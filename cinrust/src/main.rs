#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use cty::{c_char, c_ulong, c_void, c_int, c_ushort, c_long, c_uchar};
use std::ffi::{CString};
use std::ptr;

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

type LPVOID = *mut c_void;

// typedef struct tagWNDCLASSA {
//   UINT      style;
//   WNDPROC   lpfnWndProc;
//   int       cbClsExtra;
//   int       cbWndExtra;
//   HINSTANCE hInstance;
//   HICON     hIcon;
//   HCURSOR   hCursor;
//   HBRUSH    hbrBackground;
//   LPCSTR    lpszMenuName;
//   LPCSTR    lpszClassName;
// } WNDCLASSA, *PWNDCLASSA, *NPWNDCLASSA, *LPWNDCLASSA;

type UINT = u32;

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

#[repr(C)]
struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

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


extern "system" {
    fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR,
                      dwStyle: DWORD, X: i32, Y: i32, nWidth: i32, nHeight: i32,
                      hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    fn RegisterClassA(lpWndClass: * const WNDCLASSA) -> ATOM;
    fn ShowWindow(hwnd: HWND, nCmdShow: i32) -> BOOL;
    fn GetLastError() -> DWORD;
    fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

    // Message handling
    fn GetMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    fn DispatchMessageA(lpMsg: *const MSG) -> BOOL;
    fn PostQuitMessage(nExitCode: i32);

    fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    fn FillRect(hDC: HDC,lPrc:  *const RECT, hbr: HBRUSH) -> c_int;
    fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
}

fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

fn HIWORD(l: DWORD) -> WORD {
    (((l) >> 16) & 0xffff) as WORD
}

pub const WM_DESTROY: UINT = 0x0002;
pub const WM_SIZE: UINT = 0x0005;
pub const WM_PAINT: UINT = 0x000f;

pub const COLOR_WINDOW: UINT = 5;

unsafe extern "system" fn window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match u_msg {
        WM_SIZE => {
            let width = LOWORD(l_param as DWORD);
            let height = HIWORD(l_param as DWORD);
            println!("Resizing!");

            on_size(hwnd, w_param as UINT, width as c_int, height as c_int);
        },
        WM_DESTROY => {
            PostQuitMessage(0);
        },
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::new();

            let hdc = BeginPaint(hwnd, &mut ps);

            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
            EndPaint(hwnd, &mut ps);
        }
        _ => ()
    }
    DefWindowProcA(hwnd, u_msg, w_param, l_param)
}

fn on_size(hwnd: HWND, flag: UINT, width: c_int, height: c_int) {

}

pub const WS_OVERLAPPED: DWORD = 0;
pub const WS_CAPTION: DWORD = 0x00c0000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const CW_USEDEFAULT: c_int = 2147483648u32 as c_int;

fn main() {
    unsafe {
        let class_name = CString::new("Sample Window Class").expect("CString::new failed");

        let h_instance = GetModuleHandleA(ptr::null_mut());

        let mut wnd_class = WNDCLASSA::new();
        wnd_class.lpfnWndProc = Some(window_proc);
        wnd_class.hInstance = h_instance;
        wnd_class.lpszClassName = class_name.as_ptr();

        RegisterClassA(&wnd_class);
        
        let name = CString::new("Hello, world!").expect("CString::new failed");
               
        let hwnd = CreateWindowExA(0, class_name.as_ptr(), name.as_ptr(), WS_OVERLAPPEDWINDOW,
                                   CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, ptr::null_mut(),
                                   ptr::null_mut(), h_instance, ptr::null_mut());

        if hwnd.is_null() {
            let error = GetLastError();
            println!("Error: {}", error);
            return;
        }

        // 5 for SW_SHOW
        let show = ShowWindow(hwnd, 5);

        let mut msg = MSG::new();
        while GetMessageA(&mut msg, ptr::null_mut(), 0, 0) == 1 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }

}
