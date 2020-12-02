use cty::{c_char, c_void, c_long, c_int, int64_t};
//use com::interfaces::Interface;

use crate::shared::minwindef::{ ULONG };

pub type CHAR = c_char;
pub type VOID = c_void;
pub type LONG = c_long;
pub type INT = c_int;

pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;

pub type wchar_t = u16;
pub type WCHAR = wchar_t;

pub type LONGLONG = int64_t;
pub type ULONGLONG = int64_t;

// UNION! { union LARGE_INTEGER {
//     [i64; 1],
//     s s_mut: LARGE_INTEGER_s,
//     u u_mut: LARGE_INTEGER_u,
//     QuadPart QuadPart_mut: LONGLONG,
// }}

UNION! { union LARGE_INTEGER {
    s: LARGE_INTEGER_s,
    u: LARGE_INTEGER_u,
    QuadPart: LONGLONG,
}}

STRUCT! { struct LARGE_INTEGER_s {
    LowPart: ULONG,
    HighPart: LONG,
}}

STRUCT! { struct LARGE_INTEGER_u {
    LowPart: ULONG,
    HighPart: LONG,
}}


