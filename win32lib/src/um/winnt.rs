use cty::{c_char};

pub type CHAR = c_char;
pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;
