use cty::{c_ulong, c_void, c_int, c_ushort, c_uchar};
use crate::um::winnt::{ VOID, LONG, INT };

pub type DWORD = c_ulong;

pub type LPVOID = *mut VOID;
pub type UINT = u32;
pub type UINT_PTR = usize;
pub type WPARAM = UINT_PTR;

pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type INT_PTR = isize;
pub type LRESULT = INT_PTR;
pub type WORD = c_ushort;
pub type ATOM = WORD;
pub type BOOL = c_int;

opaque!(HICON, HICON__);
opaque!(HCURSOR, HCURSOR__);
opaque!(HBRUSH, HBRUSH__);
opaque!(HMONITOR, HMONITOR__);
opaque!(HWND, HWND__);
opaque!(HMENU, HMENU__);
opaque!(HINSTANCE, HINSTANCE__);
pub type HMODULE = HINSTANCE;
pub type HANDLE = *mut c_void;


pub const CS_VREDRAW: UINT = 0x0001;
pub const CS_HREDRAW: UINT = 0x0002;
pub const CS_OWNDC: UINT = 0x0020;

STRUCT! { struct POINT {
    x: LONG,
    y: LONG,
}}

impl POINT {
    pub fn new() -> POINT {
        POINT {x: 0, y: 0}
    }
}

pub type BYTE = c_uchar;

opaque!(HDC, HDC__);

opaque!(HBITMAP, HBITMAP__);

pub type HGDIOBJ = *mut c_void;

STRUCT! { struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}}

pub type LPRECT = *mut RECT;

impl RECT {
    pub fn new() -> RECT {
        RECT {left: 0, top: 0, right: 0, bottom: 0}
    }
}

pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

pub fn HIWORD(l: DWORD) -> WORD {
    (((l) >> 16) & 0xffff) as WORD
}

pub fn MAKEWORD(a: BYTE, b: BYTE) -> WORD {
    let left = ((a & 0xFF) as WORD) << 8;
    let right = (b & 0xFF) as WORD;
    (left | right) as WORD
}

pub const MB_OKCANCEL: UINT = 0x00000001;
pub const IDOK: INT = 1;

pub const COLOR_WINDOW: UINT = 5;

pub const GWLP_USERDATA: INT = -21;
