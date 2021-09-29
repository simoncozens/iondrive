use crate::{MyToPyObject, ToWrappedPyObject};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

impl MyToPyObject for norad::PointType {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            norad::PointType::Move => Some("move".to_string()),
            norad::PointType::Line => Some("line".to_string()),
            norad::PointType::OffCurve => None,
            norad::PointType::Curve => Some("curve".to_string()),
            norad::PointType::QCurve => Some("qcurve".to_string()),
        }
        .to_object(py)
    }
}

impl ToWrappedPyObject for norad::ContourPoint {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.getattr("Point").unwrap();
        let kwargs = [
            ("x", f64_to_int_or_float(self.x, py)),
            ("y", f64_to_int_or_float(self.y, py)),
            ("type", self.typ.to_object(py)),
            ("smooth", self.smooth.to_object(py)),
            ("name", self.name.to_object(py)),
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

/// Converts value to a Python integer if it can be considered one, otherwise a
/// Python float.
fn f64_to_int_or_float(v: f64, py: Python) -> PyObject {
    if (v - v.round()).abs() < std::f64::EPSILON {
        (v as i32).to_object(py)
    } else {
        v.to_object(py)
    }
}
