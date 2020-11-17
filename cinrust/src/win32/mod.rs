#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ptr;
use cty::{c_char, c_ulong, c_void, c_int, c_ushort, c_long, c_uchar};
use std::ffi::{CString};

use crate::window::*;

pub enum HWND__ {}
type HWND = *mut HWND__;

type CHAR = c_char;
type LPSTR = *mut CHAR;
type LPCSTR = * const CHAR;

type DWORD = c_ulong;

pub enum HMENU__ {}
type HMENU = *mut HMENU__;

pub enum HINSTANCE__ {}
type HINSTANCE = *mut HINSTANCE__;
type HMODULE = HINSTANCE;
type HANDLE = *mut c_void;

type VOID = c_void;
type LPVOID = *mut VOID;

type UINT = u32;
type INT = i32;

type UINT_PTR = usize;
type WPARAM = UINT_PTR;

type LONG = c_long;
type LONG_PTR = isize;
type LPARAM = LONG_PTR;

type INT_PTR = isize;
type LRESULT = INT_PTR;

type WORD = c_ushort;
type ATOM = WORD;

type BOOL = c_int;

type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

pub enum HICON__ {}
type HICON = *mut HICON__;

pub enum HCURSOR__ {}
type HCURSOR = *mut HCURSOR__;

pub enum HBRUSH__ {}
type HBRUSH = *mut HBRUSH__;

#[repr(C)]
struct WNDCLASSA {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR
}

pub const CS_VREDRAW: UINT = 0x0001;
pub const CS_HREDRAW: UINT = 0x0002;
pub const CS_OWNDC: UINT = 0x0020;

#[repr(C)]
struct POINT {
    x: LONG,
    y: LONG
}

impl POINT {
    fn new() -> POINT {
        POINT {x: 0, y: 0}
    }
}

#[repr(C)]
struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD
}

impl MSG {
    fn new() -> MSG {
        MSG {
            hwnd: ptr::null_mut(), message: 0, wParam: 0, lParam: 0,
            time: 0, pt: POINT::new(), lPrivate: 0
        }
    }
}

type LPMSG = *mut MSG;

type BYTE = c_uchar;

pub enum HDC__ {}
type HDC = *mut HDC__;

pub enum HBITMAP__ {}
type HBITMAP = *mut HBITMAP__;

#[repr(C)]
struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

type LPRECT = *mut RECT;

impl RECT {
    fn new() -> RECT {
        RECT {left: 0, top: 0, right: 0, bottom: 0}
    }
}

#[repr(C)]
struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore : BOOL,
    pub fIncUpdate: BOOL,
    pub gbReserved: [BYTE; 32]
}

impl PAINTSTRUCT {
    fn new() -> PAINTSTRUCT {
        PAINTSTRUCT {
            hdc: ptr::null_mut(),
            fErase: 0,
            rcPaint: RECT::new(),
            fRestore: 0,
            fIncUpdate: 0,
            gbReserved: [0; 32]
        }
    }
}

type LPPAINTSTRUCT = *mut PAINTSTRUCT;

impl WNDCLASSA {
    fn new() -> WNDCLASSA {
        WNDCLASSA { style: 0, lpfnWndProc: None, cbClsExtra: 0, cbWndExtra: 0,
                   hInstance: ptr::null_mut(), hIcon: ptr::null_mut(), hCursor: ptr::null_mut(),
                   hbrBackground: ptr::null_mut(), lpszMenuName: ptr::null_mut(), lpszClassName: ptr::null_mut()}
    }
}

#[repr(C)]
#[derive(Default)]
struct BITMAPINFOHEADER {
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
#[derive(Default)]
struct RGBQUAD {
  rgbBlue:     BYTE, 
  rgbGreen:    BYTE, 
  rgbRed:      BYTE, 
  rgbReserved: BYTE
}


#[repr(C)]
struct BITMAPINFO {
  bmiHeader: BITMAPINFOHEADER,
  bmiColors: [RGBQUAD; 1]
}

extern "system" {
    fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR,
                      dwStyle: DWORD, X: i32, Y: i32, nWidth: i32, nHeight: i32,
                      hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    fn RegisterClassA(lpWndClass: * const WNDCLASSA) -> ATOM;
    fn ShowWindow(hwnd: HWND, nCmdShow: i32) -> BOOL;

    fn CreateCompatibleDC(hdc: HDC) -> HDC;
    fn GetLastError() -> DWORD;
    fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn DestroyWindow(hWnd: HWND) -> BOOL;
    fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR);
    fn GetWindowLongPtrA(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    fn CreateDIBSection(hdc: HDC, pbmi: *const BITMAPINFO, usage: UINT, ppvbits: *mut *mut VOID, hSection: HANDLE, offset: DWORD) -> HBITMAP;

    fn StretchDIBits(hdc: HDC, xDest: c_int, yDest: c_int, DestWidth: c_int, DestHeight: c_int,
                     xSrc: c_int, ySrc: c_int, SrcWidth: c_int, SrcHeight: c_int, lpBits: *const VOID,
                     lpbmi: *const BITMAPINFO, iUsage: UINT, rop: DWORD) -> c_int;


    // Message handling
    fn GetMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    fn DispatchMessageA(lpMsg: *const MSG) -> BOOL;
    fn PostQuitMessage(nExitCode: c_int);
    fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;

    fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    fn FillRect(hDC: HDC,lPrc:  *const RECT, hbr: HBRUSH) -> c_int;
    fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

    fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
}

fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

fn HIWORD(l: DWORD) -> WORD {
    (((l) >> 16) & 0xffff) as WORD
}

fn MAKEWORD(a: BYTE, b: BYTE) -> WORD {
    let left = ((a & 0xFF) as WORD) << 8;
    let right = (b & 0xFF) as WORD;
    (left | right) as WORD
}

fn RGB(r: BYTE, g: BYTE, b: BYTE) -> DWORD {
    // Reversed due to endianness of FillRect
    let left = MAKEWORD(0x00, b) as DWORD;
    let right = MAKEWORD(g, r) as DWORD;
    ((left << 16) | right) as DWORD
}

type COLORREF = DWORD;
type LPCOLORREf = *mut DWORD;

const RGB_RED: COLORREF  =  0x000000FF;
const RGB_GREEN: COLORREF =  0x0000FF00;
const RGB_BLUE: COLORREF  =  0x00FF0000;
const RGB_BLACK: COLORREF =  0x00000000;
const RGB_WHITE: COLORREF =  0x00FFFFFF;

const WM_DESTROY: UINT = 0x0002;
const WM_SIZE: UINT = 0x0005;
const WM_PAINT: UINT = 0x000f;
const WM_CLOSE: UINT = 0x0010;

const MB_OKCANCEL: UINT = 0x00000001;
const IDOK: INT = 1;

const COLOR_WINDOW: UINT = 5;

pub struct Win32Window {
    pub hwnd: HWND,
    user_data: WindowPtrData
}

fn set_running(hwnd: HWND, running: bool) {
    unsafe {
        let mut data = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut WindowPtrData;
        if !data.is_null() {
            (*data).running = running;
            SetWindowLongPtrA(hwnd, GWLP_USERDATA, data as isize);
        }
    }
}

fn get_window_ptr(hwnd: HWND) -> Option<WindowPtrData> {
    let data = 
    unsafe {
        let data = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut WindowPtrData;
        if data.is_null() {
            Some(*data)
        }
        else {
            None
        }
    };
    data
}

pub const GWLP_USERDATA: INT = -21;

unsafe extern "system" fn window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let result = match u_msg {
        WM_SIZE => {
            let mut rect = RECT::new();
            GetClientRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            on_size(hwnd, width, height);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        WM_DESTROY => {
            set_running(hwnd, false);
            0
        },
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::new();

            let hdc = BeginPaint(hwnd, &mut ps);
            
            let color = RGB(200, 0, 127);
            let color_brush = CreateSolidBrush(color);

            FillRect(hdc, &ps.rcPaint, color_brush);
            EndPaint(hwnd, &mut ps);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        WM_CLOSE => {
            set_running(hwnd, false);
            0
        },
        _ => DefWindowProcA(hwnd, u_msg, w_param, l_param)
    };
    result
}

pub const DIB_RGB_COLORS: UINT = 0;

pub const SRCCOPY: DWORD = 0xCC0020;

pub const BI_RGB: UINT = 0;

fn on_size(h_wnd: HWND, width: c_int, height: c_int) {
    unsafe {
        let bitmap_info_header: BITMAPINFOHEADER = Default::default();
        bitmap_info_header.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_info_header.biWidth = width;
        bitmap_info_header.biHeight = height;
        bitmap_info_header.biPlanes = 1;
        bitmap_info_header.biBitCount = 32;
        bitmap_info_header.biCompression = BI_RGB;
        bitmap_info_header.biSizeImage = 0;
        bitmap_info_header.biXPelsPerMeter = 0;
        bitmap_info_header.biYPelsPerMeter = 0;
        bitmap_info_header.biClrUsed = 0;
        bitmap_info_header.biClrImportant = 0;
        let bitmap_info = BITMAPINFO { bmiHeader: bitmap_info_header, bmiColors: [Default::default()]};
        let mut bitmap_memory: [u32; 800 * 600] = [0; 800 * 600];

        let hdc = CreateCompatibleDC(ptr::null_mut());
        
        let bitmap = CreateDIBSection(hdc, &mut bitmap_info, DIB_RGB_COLORS, bitmap_memory.as_mut_ptr() as *mut *mut c_void,
                                      ptr::null_mut(), 0);
    }
}

fn update_window(hdc: HDC, x: c_int, y: c_int, width: c_int, height: c_int) {
    unsafe {
        StretchDIBits(hdc,
                      x, y, width, height,
                      x, y, width, height,
                      bits, bits_info,
                      DIB_RGB_COLORS, SRCCOPY
        );
    }
}

pub const WS_OVERLAPPED: DWORD = 0;
pub const WS_CAPTION: DWORD = 0x00c0000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const CW_USEDEFAULT: c_int = 2147483648u32 as c_int;


impl Window for Win32Window {
    fn create_window(name: &str, width: i32, height: i32) -> Win32Window {
        let class_name = CString::new("Sample Window Class").expect("CString::new failed");
        let name = CString::new("Hello, world!").expect("CString::new failed");
        
        unsafe {
            let h_instance = GetModuleHandleA(ptr::null_mut());
            let mut wnd_class = WNDCLASSA::new();
            wnd_class.lpfnWndProc = Some(window_proc);
            wnd_class.hInstance = h_instance;
            wnd_class.lpszClassName = class_name.as_ptr();
            wnd_class.style = CS_OWNDC | CS_VREDRAW | CS_HREDRAW;

            RegisterClassA(&wnd_class);
            
            let hwnd = CreateWindowExA(0, class_name.as_ptr(), name.as_ptr(), WS_OVERLAPPEDWINDOW,
                                       CW_USEDEFAULT, CW_USEDEFAULT, width as c_int, height as c_int, ptr::null_mut(),
                                       ptr::null_mut(), h_instance, ptr::null_mut());

            if hwnd.is_null() {
                let error = GetLastError();
                println!("Error: {}", error);
                Win32Window { hwnd, user_data: WindowPtrData { running: false} };
            }

            // 5 for SW_SHOW
            let show = ShowWindow(hwnd, 5);

            let user_data = WindowPtrData { running: true };
            
            let window = Win32Window { hwnd, user_data };

            SetWindowLongPtrA(hwnd, GWLP_USERDATA, (&window.user_data as *const WindowPtrData) as isize);
            window
        }
    }

    fn is_open(&self) -> bool {
        unsafe {
            let data = GetWindowLongPtrA(self.hwnd, GWLP_USERDATA) as *const WindowPtrData;
            if data.is_null() {
                false
            } else {
                (*data).running
            }
        }
    }

    fn poll_events(&self) {
        unsafe {
            let mut msg = MSG::new();
            GetMessageA(&mut msg, ptr::null_mut(), 0, 0);
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

pub fn create_window(name: &str, width: i32, height: i32) -> Win32Window {
    Win32Window::create_window(name, width, height)
}
