use cty::{c_int};
use crate::shared::windef::{BOOL, HGDIOBJ, UINT, HANDLE, DWORD, HBITMAP, HDC, WORD, BYTE,
                            MAKEWORD, HBRUSH};
use crate::um::winnt::{VOID, LONG};

pub fn RGB(r: BYTE, g: BYTE, b: BYTE) -> DWORD {
    // Reversed due to endianness of FillRect
    let left = MAKEWORD(0x00, b) as DWORD;
    let right = MAKEWORD(g, r) as DWORD;
    ((left << 16) | right) as DWORD
}

pub type COLORREF = DWORD;
pub type LPCOLORREf = *mut DWORD;

pub const RGB_RED: COLORREF  =  0x000000FF;
pub const RGB_GREEN: COLORREF =  0x0000FF00;
pub const RGB_BLUE: COLORREF  =  0x00FF0000;
pub const RGB_BLACK: COLORREF =  0x00000000;
pub const RGB_WHITE: COLORREF =  0x00FFFFFF;

#[repr(C)]
#[derive(Default, Debug)]
pub struct BITMAPINFOHEADER {
  pub biSize:        DWORD, 
  pub biWidth:       LONG , 
  pub biHeight:      LONG , 
  pub biPlanes:      WORD , 
  pub biBitCount:    WORD , 
  pub biCompression: DWORD, 
  pub biSizeImage:   DWORD, 
  pub biXPelsPerMeter: LONG,
  pub biYPelsPerMeter: LONG,
  pub biClrUsed:     DWORD, 
  pub biClrImportant: DWORD 
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct RGBQUAD {
  rgbBlue:     BYTE, 
  rgbGreen:    BYTE, 
  rgbRed:      BYTE, 
  rgbReserved: BYTE
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct BITMAPINFO {
  pub bmiHeader: BITMAPINFOHEADER,
  pub bmiColors: [RGBQUAD; 1]
}

extern "system" {
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;
    
    pub fn CreateDIBSection(hdc: HDC, pbmi: *const BITMAPINFO, usage: UINT, ppvbits: *mut *mut VOID, hSection: HANDLE, offset: DWORD) -> HBITMAP;
    pub fn StretchDIBits(hdc: HDC, xDest: c_int, yDest: c_int, DestWidth: c_int, DestHeight: c_int,
                     xSrc: c_int, ySrc: c_int, SrcWidth: c_int, SrcHeight: c_int, lpBits: *const VOID,
                     lpbmi: *const BITMAPINFO, iUsage: UINT, rop: DWORD) -> c_int;
}
