use cty::{c_int};
use std::ptr;

use crate::shared::windef::{HICON, HCURSOR, BYTE, DWORD, HINSTANCE, HWND, HMENU, LPVOID, BOOL, HDC,
             UINT, ATOM, WPARAM, LPARAM, LRESULT, HMODULE, LONG_PTR, LPRECT,
                            HBRUSH, RECT, POINT};
use crate::um::winnt::{LPCSTR};

pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;
pub const PM_NOYIELD: UINT = 0x0002;

pub const WS_OVERLAPPED: DWORD = 0;
pub const WS_CAPTION: DWORD = 0x00c0000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const CW_USEDEFAULT: c_int = 2147483648u32 as c_int;
#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD
}

impl MSG {
    pub fn new() -> MSG {
        MSG {
            hwnd: ptr::null_mut(), message: 0, wParam: 0, lParam: 0,
            time: 0, pt: POINT::new(), lPrivate: 0
        }
    }
}

pub type LPMSG = *mut MSG;


type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;
#[repr(C)]
pub struct WNDCLASSA {
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

impl WNDCLASSA {
    pub fn new() -> WNDCLASSA {
        WNDCLASSA { style: 0, lpfnWndProc: None, cbClsExtra: 0, cbWndExtra: 0,
                   hInstance: ptr::null_mut(), hIcon: ptr::null_mut(), hCursor: ptr::null_mut(),
                   hbrBackground: ptr::null_mut(), lpszMenuName: ptr::null_mut(), lpszClassName: ptr::null_mut()}
    }
}

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore : BOOL,
    pub fIncUpdate: BOOL,
    pub gbReserved: [BYTE; 32]
}

impl PAINTSTRUCT {
    pub fn new() -> PAINTSTRUCT {
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

pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

pub mod notifications {
    use super::UINT;
    
    pub const WM_NULL: UINT =                   0x0000;
    pub const WM_CREATE: UINT =                 0x0001;
    pub const WM_DESTROY: UINT =                0x0002;
    pub const WM_MOVE: UINT =                   0x0003;
    pub const WM_SIZE: UINT =                   0x0005;
    pub const WM_ACTIVATE: UINT =               0x0006;
    pub const WM_SETFOCUS: UINT =               0x0007;
    pub const WM_ENABLED: UINT =                0x000A;
    pub const WM_PAINT: UINT =                  0x000F;
    pub const WM_CLOSE: UINT =                  0x0010;
    pub const WM_KEYDOWN: UINT =                0x0100;
    pub const WM_KEYUP: UINT =                  0x0101; 
    pub const WM_QUIT: UINT =                   0x0012;
    pub const WM_QUERYOPEN: UINT =              0x0013;
    pub const WM_ERASEBKGND: UINT =             0x0014;
    pub const WM_SHOWWINDOW: UINT =             0x0018;
    pub const WM_ACTIVATEAPP: UINT =            0x001C;
    pub const WM_CANCELMODE: UINT =             0x001F;
    pub const WM_CHILDACTIVATE: UINT =          0x0022;
    pub const WM_GETMINMAXINFO: UINT =          0x0024;
    pub const WM_QUERYDRAGICON: UINT =          0x0037;
    pub const WM_COMPACTING: UINT =             0x0041;
    pub const WM_WINDOWPOSCHANGING: UINT =      0x0046;
    pub const WM_WINDOWPOSCHANGED: UINT =       0x0047;
    pub const WM_INPUTLANGCHANGEREQUEST: UINT = 0x0050;
    pub const WM_INPUTLANGCHANGE: UINT =        0x0051;
    pub const WM_USERCHANGED: UINT =            0x0054;
    pub const WM_STYLECHANGING: UINT =          0x007C;
    pub const WM_STYLECHANGED: UINT =           0x007D;
    pub const WM_GETICON: UINT =                0x007F;
    pub const WM_NCCREATE: UINT =               0x0081;
    pub const WM_NCDESTROY: UINT =              0x0082;
    pub const WM_NCALCSIZE: UINT =              0x0083;
    pub const WM_NCPAINT: UINT =                0x0085;
    pub const WM_NACTIVATE: UINT =              0x0086;
    pub const WM_SIZING: UINT =                 0x0214;
    pub const WM_MOVING: UINT =                 0x0216;
    pub const WM_ENTERSIZEMOVE: UINT =          0x0231;
    pub const WM_EXITSIZEMOVE: UINT =           0x0232;
    pub const WM_IME_SETCONTEXT: UINT =         0x0281;
    pub const WM_IME_NOTIFY: UINT =             0x0282;
    pub const WM_DPICHANGED: UINT =             0x02E0;
    pub const WM_THEMECHANGED: UINT =           0x031A;
}


extern "system" {
    pub fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR,
                           dwStyle: DWORD, X: i32, Y: i32, nWidth: i32, nHeight: i32,
                           hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    pub fn IsWindow(hwnd: HWND) -> BOOL;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn RegisterClassA(lpWndClass: * const WNDCLASSA) -> ATOM;
    pub fn ShowWindow(hwnd: HWND, nCmdShow: i32) -> BOOL;
    
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;

    pub fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn GetWindowLongPtrA(hWnd: HWND, nIndex: c_int) -> LONG_PTR;

    pub fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;



    // Message handling
    pub fn GetMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn PeekMessageA(lpMSG: LPMSG, hwnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> BOOL;

    
    pub fn DispatchMessageA(lpMsg: *const MSG) -> BOOL;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;

    pub fn BeginPaint(hwnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
    pub fn FillRect(hDC: HDC,lPrc:  *const RECT, hbr: HBRUSH) -> c_int;
    pub fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
}
