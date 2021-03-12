use pyo3::types::IntoPyDict;
use pyo3::types::PyDict;
use pyo3::types::PyList;

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;
mod anchor;
mod component;
mod contour;
mod contourpoint;
mod guideline;
mod info;
mod plist;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyUnicode;
use pyo3::wrap_pyfunction;

static DEFAULT_GLYPHS_DIRNAME: &str = "glyphs";

trait ToWrappedPyObject {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject;
}

trait MyToPyObject {
    fn to_object(&self, py: Python) -> PyObject;
}

impl<T> ToWrappedPyObject for Option<T>
where
    T: ToWrappedPyObject,
{
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        return self
            .as_ref()
            .map_or(py.None(), |x| x.to_wrapped_object(loader, py));
    }
}

impl<T> ToWrappedPyObject for Vec<T>
where
    T: ToWrappedPyObject,
{
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        self.iter()
            .map(|x| x.to_wrapped_object(loader, py))
            .collect::<Vec<PyObject>>()
            .to_object(py)
    }
}

impl<T> MyToPyObject for Vec<T>
where
    T: MyToPyObject,
{
    fn to_object(&self, py: Python) -> PyObject {
        self.iter()
            .map(|x| x.to_object(py))
            .collect::<Vec<PyObject>>()
            .to_object(py)
    }
}
impl<T> MyToPyObject for Option<T>
where
    T: MyToPyObject,
{
    fn to_object(&self, py: Python) -> PyObject {
        self.as_ref().map_or(py.None(), |x| x.to_object(py))
    }
}

impl MyToPyObject for Arc<str> {
    fn to_object(&self, py: Python) -> PyObject {
        Arc::clone(self).to_string().to_object(py)
    }
}

impl<A, B> MyToPyObject for BTreeMap<A, B>
where
    A: ToPyObject + std::fmt::Debug,
    B: MyToPyObject + std::fmt::Debug,
{
    fn to_object(&self, py: Python) -> PyObject {
        let d = PyDict::new(py);
        for (k, v) in self.iter() {
            d.set_item(k, v.to_object(py)).unwrap();
        }
        d.into()
    }
}

impl ToWrappedPyObject for Arc<norad::Glyph> {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.get("Glyph").unwrap();
        let kwargs = [
            ("name", self.name.to_object(py)),
            ("width", self.advance_width().unwrap_or(0.0).to_object(py)),
            (
                "unicodes",
                self.codepoints
                    .as_ref()
                    .map_or(PyList::empty(py).to_object(py), |cp| {
                        cp.iter()
                            .map(|l| (*l as u32).to_object(py))
                            .collect::<Vec<PyObject>>()
                            .to_object(py)
                    }),
            ),
            (
                "lib",
                self.lib
                    .as_ref()
                    .map_or(PyDict::new(py).to_object(py), |l| l.to_object(py)),
            ),
            ("note", self.note.to_object(py)),
            (
                "anchors",
                self.anchors
                    .as_ref()
                    .map_or(PyList::empty(py).to_object(py), |a| {
                        a.to_wrapped_object(loader, py)
                    }),
            ),
            (
                "contours",
                self.outline
                    .as_ref()
                    .map_or(PyList::empty(py).to_object(py), |c| {
                        c.contours.to_wrapped_object(loader, py)
                    }),
            ),
            (
                "components",
                self.outline
                    .as_ref()
                    .map_or(PyList::empty(py).to_object(py), |c| {
                        c.components.to_wrapped_object(loader, py)
                    }),
            ),
            (
                "guidelines",
                self.guidelines
                    .as_ref()
                    .map_or(PyList::empty(py).to_object(py), |g| {
                        g.to_wrapped_object(loader, py)
                    }),
            ),
        ]
        .into_py_dict(py);
        cls.call((), Some(kwargs)).unwrap().into()
    }
}

impl ToWrappedPyObject for norad::LayerInfo {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.get("Layer").unwrap();
        let kwargs = [
            ("name", self.name.to_object(py)),
            (
                "glyphs",
                self.layer
                    .iter_contents()
                    .map(|l| l.to_wrapped_object(loader, py))
                    .collect::<Vec<PyObject>>()
                    .to_object(py),
            ),
            // ("lib", self.layer.lib.to_object(py)),
            (
                "color",
                self.layer
                    .info
                    .color
                    .as_ref()
                    .map(|c| c.to_rgba_string())
                    .to_object(py),
            ), // ()
        ]
        .into_py_dict(py);
        cls.call((), Some(kwargs)).unwrap().into()
    }
}

fn wrap_layerset(
    layers: &Vec<norad::LayerInfo>,
    default_layer_name: Option<&String>,
    loader: &PyModule,
    py: Python,
) -> PyObject {
    let cls = loader.get("LayerSet").unwrap();
    cls.call_method(
        "from_iterable",
        ((*layers).to_wrapped_object(loader, py), default_layer_name),
        None,
    )
    .unwrap()
    .into()
}

fn wrap_kerning(kerning: Option<&BTreeMap<String, BTreeMap<String, f32>>>, py: Python) -> PyObject {
    if kerning.is_none() {
        return py.None();
    }
    let d = PyDict::new(py);
    for (left, v) in kerning.unwrap().iter() {
        for (right, kern) in v.iter() {
            d.set_item((left, right).to_object(py), kern.to_object(py));
        }
    }
    d.into()
}
impl ToWrappedPyObject for norad::Ufo {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let font = loader.get("Font").unwrap();
        let default_layer_name = self
            .layers
            .iter()
            .find(|l| l.path.file_name() == Some(OsStr::new(DEFAULT_GLYPHS_DIRNAME)))
            .map(|l| &l.name);

        let kwargs = [
            ("lib", self.lib.to_object(py)),
            (
                "layers",
                wrap_layerset(self.layers.as_ref(), default_layer_name, loader, py),
            ),
            ("info", self.font_info.to_wrapped_object(loader, py)),
            ("features", pyo3::ToPyObject::to_object(&self.features, py)),
            ("groups", self.groups.to_object(py)),
            ("kerning", wrap_kerning(self.kerning.as_ref(), py)),
        ]
        .into_py_dict(py);
        font.call((), Some(kwargs)).unwrap().into()
    }
}

#[pyfunction]
fn load(loader: &PyModule, path: &PyUnicode) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let s: String = path.extract()?;
    match norad::Ufo::load(Path::new(&s)) {
        Ok(ufo) => Ok(ufo.to_wrapped_object(loader, py)),
        Err(error) => Err(PyValueError::new_err(error.to_string())),
    }
}

#[pymodule]
fn iondrive(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?).unwrap();

    Ok(())
}
