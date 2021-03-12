use crate::MyToPyObject;
use pyo3::prelude::*;
use pyo3::types::PyDict;

impl MyToPyObject for plist::Value {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            plist::Value::String(s) => s.to_object(py),
            plist::Value::Boolean(s) => s.to_object(py),
            plist::Value::Data(s) => s.to_object(py),
            plist::Value::Real(s) => s.to_object(py),
            plist::Value::Integer(s) => s.as_signed().to_object(py),
            plist::Value::Uid(s) => s.get().to_object(py),
            plist::Value::Array(s) => s
                .iter()
                .map(|v| v.to_object(py))
                .collect::<Vec<PyObject>>()
                .to_object(py),
            plist::Value::Dictionary(s) => s.to_object(py),
            // XXX Date!
            _ => py.None(),
        }
    }
}

impl MyToPyObject for plist::Dictionary {
    fn to_object(&self, py: Python) -> PyObject {
        let d = PyDict::new(py);
        for (k, v) in self.iter() {
            d.set_item(k, v.to_object(py)).unwrap();
        }
        d.into()
    }
}
