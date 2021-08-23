use crate::ToWrappedPyObject;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

impl ToWrappedPyObject for norad::Contour {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.getattr("Contour").unwrap();
        let kwargs = [
            ("points", self.points.to_wrapped_object(loader, py)),
            (
                "identifier",
                self.identifier()
                    .map_or(py.None(), |i| i.as_str().to_object(py)),
            ),
        ]
        .into_py_dict(py);
        cls.call((), Some(kwargs)).unwrap().into()
    }
}
