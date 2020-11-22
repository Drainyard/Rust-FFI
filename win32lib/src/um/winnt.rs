use cty::{c_char, c_void, c_long, c_int};

pub type CHAR = c_char;
pub type VOID = c_void;
pub type LONG = c_long;
pub type INT = c_int;

pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;

