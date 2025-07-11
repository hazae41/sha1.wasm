#![allow(unused_imports, unused_macros)]

macro_rules! rjse {
    ($x:expr) => {
        $x.map_err(|e| JsError::new(&format!("{} at {}", e, std::any::type_name_of_val(&|| {}))))
    };
}

macro_rules! ojse {
    ($x:expr) => {
        $x.ok_or_else(|| JsError::new(&format!("Option was None at {}", std::any::type_name_of_val(&|| {}))))
    };
}

pub(crate) use ojse;
pub(crate) use rjse;
