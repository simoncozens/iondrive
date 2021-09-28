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

pub fn dump_glyph_object_libs(glyph: &norad::Glyph) -> norad::Plist {
    let mut object_libs = norad::Plist::default();

    let mut dump_lib = |id: Option<&norad::Identifier>, lib: &norad::Plist| {
        let id = id.map(|id| id.as_str().to_string());
        object_libs.insert(id.unwrap(), plist::Value::Dictionary(lib.clone()));
    };

    for anchor in &glyph.anchors {
        if let Some(lib) = anchor.lib() {
            dump_lib(anchor.identifier(), lib);
        }
    }

    for guideline in &glyph.guidelines {
        if let Some(lib) = guideline.lib() {
            dump_lib(guideline.identifier(), lib);
        }
    }

    for contour in &glyph.contours {
        if let Some(lib) = contour.lib() {
            dump_lib(contour.identifier(), lib);
        }
        for point in &contour.points {
            if let Some(lib) = point.lib() {
                dump_lib(point.identifier(), lib);
            }
        }
    }
    for component in &glyph.components {
        if let Some(lib) = component.lib() {
            dump_lib(component.identifier(), lib);
        }
    }

    object_libs
}

pub fn dump_fontinfo_object_libs(fontinfo: &norad::FontInfo) -> norad::Plist {
    let mut object_libs = norad::Plist::default();

    if let Some(guidelines) = &fontinfo.guidelines {
        for guideline in guidelines {
            if let Some(lib) = guideline.lib() {
                let id = guideline.identifier().map(|id| id.as_str().to_string());
                object_libs.insert(id.unwrap(), plist::Value::Dictionary(lib.clone()));
            }
        }
    }

    object_libs
}
