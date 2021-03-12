use crate::ToWrappedPyObject;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

impl ToWrappedPyObject for norad::Component {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.get("Component").unwrap();
        let kwargs = [
            ("baseGlyph", self.base.to_object(py)),
            (
                "transformation",
                vec![
                    self.transform.x_scale,
                    self.transform.xy_scale,
                    self.transform.yx_scale,
                    self.transform.y_scale,
                    self.transform.x_offset,
                    self.transform.y_offset,
                ]
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
