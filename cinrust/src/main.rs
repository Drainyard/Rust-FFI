use cty::{c_char, c_ulong, c_void, c_int};
use std::ffi::{CString};
use std::ptr;

pub enum HWND__ {}
type HWND = *mut HWND__;

type CHAR = c_char;
type LPSTR = *mut CHAR;

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

type LONG_PTR = isize;
type LPARAM = LONG_PTR;

type INT_PTR = isize;
type LRESULT = INT_PTR;

type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

pub enum HICON__ {}
type HICON = *mut HICON__;

pub enum HCURSOR__ {}
type HCURSOR = *mut HCURSOR__;

pub enum HBRUSH__ {}
type HBRUSH = *mut HBRUSH__;


struct WNDCLASS {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPSTR,
    pub lpszClassName: LPSTR
}

impl WNDCLASS {
    fn new() -> WNDCLASS {
        unsafe {
            WNDCLASS { style: 0, lpfnWndProc: None, cbClsExtra: 0, cbWndExtra: 0,
                       hInstance: ptr::null_mut(), hIcon: ptr::null_mut(), hCursor: ptr::null_mut(),
                       hbrBackground: ptr::null_mut(), lpszMenuName: ptr::null_mut(), lpszClassName: ptr::null_mut()}
        }
    }
}


extern "C" {
    fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPSTR, lpWindowName: LPSTR,
                      dwStyle: DWORD, X: i32, Y: i32, nWidth: i32, nHeight: i32,
                      hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    fn GetModuleHandleA(lpModuleName: LPSTR) -> HMODULE;
}

unsafe extern "system" fn WindowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    0
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
        let class_raw = class_name.into_raw();

        let hInstance = GetModuleHandleA(ptr::null_mut());

        let mut wnd_class = WNDCLASS::new();
        wnd_class.lpfnWndProc = Some(WindowProc);
        wnd_class.hInstance = hInstance;
        wnd_class.lpszClassName = class_raw;
        
        let name = CString::new("Hello, world!").expect("CString::new failed");
               
        CreateWindowExA(0, class_raw, name.into_raw(), WS_OVERLAPPEDWINDOW,
                       CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, ptr::null_mut(),
                       ptr::null_mut(), hInstance, ptr::null_mut());
    }

}
