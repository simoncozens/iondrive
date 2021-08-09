use crate::ToWrappedPyObject;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

impl ToWrappedPyObject for norad::Guideline {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.getattr("Guideline").unwrap();
        let (x, y, angle) = match self.line {
            norad::Line::Vertical(x) => (Some(x), None, None),
            norad::Line::Horizontal(y) => (None, Some(y), None),
            norad::Line::Angle { x, y, degrees } => (Some(x), Some(y), Some(degrees)),
        };
        let kwargs = [
            ("name", self.name.to_object(py)),
            ("x", x.to_object(py)),
            ("y", y.to_object(py)),
            ("angle", angle.to_object(py)),
            (
                "color",
                self.color
                    .as_ref()
                    .map(|c| c.to_rgba_string())
                    .to_object(py),
            ),
            (
                "identifier",
                self.identifier()
                    .map_or(py.None(), |i| i.as_str().to_object(py)),
            ),
            // ("lib", self.lib().map_or(py.None(), |l| l.to_object(py))),
        ]
        .into_py_dict(py);
        cls.call((), Some(kwargs)).unwrap().into()
    }
}
