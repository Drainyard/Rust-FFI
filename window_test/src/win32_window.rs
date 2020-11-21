#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::window::*;

use win32lib::shared::windef::{DWORD, HDC, HBITMAP, HWND, GWLP_USERDATA, UINT, WPARAM, LPARAM, LRESULT, RECT,
             BOOL, CS_OWNDC, CS_VREDRAW, CS_HREDRAW, HGDIOBJ};
use win32lib::um::winuser::{SetLastError, SetWindowLongPtrA, GetWindowLongPtrA, GetClientRect, GetLastError,
                            DefWindowProcA, BeginPaint, EndPaint, PostQuitMessage, FillRect, CreateCompatibleDC,
                            GetModuleHandleA, CreateWindowExA, RegisterClassA, ShowWindow, PeekMessageA,
                            GetMessageA, DispatchMessageA, TranslateMessage};
use win32lib::um::winuser::notifications::*;

use win32lib::um::winuser::{PAINTSTRUCT, WNDCLASSA, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, PM_NOREMOVE, MSG};

use win32lib::um::wingdi::{CreateSolidBrush, DeleteObject, CreateDIBSection};
use win32lib::um::wingdi::{BITMAPINFO, RGB, BITMAPINFOHEADER};

use std::ptr;
use cty::{c_void, c_int};
use std::ffi::{CString};



#[repr(C)]
#[derive(Debug)]
struct Win32WindowState {
    bitmap_device_context: HDC,
    bitmap_handle: HBITMAP,
    bitmap_info: BITMAPINFO,

    bitmap_memory: *mut u32,
}

impl Win32WindowState {
    fn new() -> Win32WindowState {
        Win32WindowState {
            bitmap_device_context: ptr::null_mut(),
            bitmap_handle: ptr::null_mut(),
            bitmap_info: Default::default(),
            bitmap_memory: ptr::null_mut(),
        }
    }
}

pub struct Win32Window {
    pub hwnd: HWND
}

fn print_last_error(file: &'static str, line: u32) {
    let error_code = unsafe {
        GetLastError()
    };
    println!("Error in call in file {} on line {} : {}", file, line, error_code);
}

fn set_window_ptr(hwnd: HWND, win_state: &Win32WindowState) {
    unsafe {
        SetLastError(0);
        let result = SetWindowLongPtrA(hwnd, GWLP_USERDATA, (win_state as *const Win32WindowState) as isize);
        if result == 0 {
            print_last_error(file!(), line!());
        }
    }
}

fn get_window_ptr<'a>(hwnd: HWND) -> Option<&'a mut Win32WindowState> {
    unsafe {
        SetLastError(0);
        let data = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut Win32WindowState;
        if !data.is_null() {
            let data: &'a mut Win32WindowState = &mut *data;
            Some(data)
        }
        else {
            None
        }
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let result: LRESULT = match u_msg {
        WM_SIZE => {
            let mut rect = RECT::new();
            GetClientRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            // on_size(hwnd, width, height);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        },
        WM_CREATE => {
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
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
            PostQuitMessage(0);
            0
        },
        WM_KEYDOWN => {
            println!("KEYDOWN: {}", w_param);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        WM_KEYUP => {
            println!("KEYUP: {}", w_param);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        _ => DefWindowProcA(hwnd, u_msg, w_param, l_param)
    };
    println!("0x{} {}", format!("{:01$x}", u_msg, 4), result);
    result
}

pub const DIB_RGB_COLORS: UINT = 0;

pub const SRCCOPY: DWORD = 0xCC0020;

pub const BI_RGB: UINT = 0;

fn delete_object(ptr: *mut c_void) -> BOOL {
    unsafe {
        DeleteObject(ptr)
    }
}

fn on_size(h_wnd: HWND, width: c_int, height: c_int) {
    if let Some(mut window_state) = get_window_ptr(h_wnd) {
        if !window_state.bitmap_handle.is_null() {
            delete_object(window_state.bitmap_handle as HGDIOBJ);
        }

        if window_state.bitmap_device_context.is_null() {
            window_state.bitmap_device_context = unsafe {
                CreateCompatibleDC(ptr::null_mut())
            };
        }

        let mut bitmap_info_header: BITMAPINFOHEADER = Default::default();
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
        let mut bitmap_info = BITMAPINFO { bmiHeader: bitmap_info_header, bmiColors: [Default::default()]};
        let mut bitmap_memory: [u32; 800 * 600] = [0; 800 * 600];
        window_state.bitmap_memory = bitmap_memory.as_mut_ptr();
        
        unsafe {
            window_state.bitmap_handle = CreateDIBSection(window_state.bitmap_device_context, &mut bitmap_info, DIB_RGB_COLORS, bitmap_memory.as_mut_ptr() as *mut *mut c_void, ptr::null_mut(), 0);
        }

        set_window_ptr(h_wnd, window_state);
    }
}

fn update_window(h_wnd: HWND, x: c_int, y: c_int, width: c_int, height: c_int) {
    if let Some(window_state) = get_window_ptr(h_wnd) {
        // unsafe {
        //     StretchDIBits(hdc,
        //                   x, y, width, height,
        //                   x, y, width, height,
        //                   window_state.bitmap_memory.as_mut_ptr(), window_state.bitmap_info.as_mut_ptr(),
        //                   DIB_RGB_COLORS, SRCCOPY
        //     );
        // }
    }
    
}


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

            let result = RegisterClassA(&wnd_class);
            if result == 0 {
                print_last_error(file!(), line!());
                let window_state = Win32WindowState::new();
                return Win32Window { hwnd: 0 as HWND};
            }
            
            let hwnd = CreateWindowExA(0, class_name.as_ptr(), name.as_ptr(), WS_OVERLAPPEDWINDOW,
                                       CW_USEDEFAULT, CW_USEDEFAULT, width as c_int, height as c_int, ptr::null_mut(),
                                       ptr::null_mut(), h_instance, ptr::null_mut());

            if hwnd.is_null() {
                print_last_error(file!(), line!());

                return Win32Window { hwnd};
            }

            // 5 for SW_SHOW
            let show = ShowWindow(hwnd, 5);

            let window = Win32Window { hwnd};

            window
        }
    }

    fn is_open(&self) -> bool {
        unsafe {
            let mut msg = MSG::new();
            let peek = PeekMessageA(&mut msg, 0 as HWND, 0, 0, PM_NOREMOVE);

            match msg.message {
                WM_DESTROY | WM_QUIT => {
                    false
                },
                _ => true
            }
        }
    }

    fn poll_events(&self) {
        unsafe {
            let mut msg = MSG::new();
            let message = GetMessageA(&mut msg, 0 as HWND, 0, 0);

            if message > 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }
}

pub fn create_window(name: &str, width: i32, height: i32) -> Win32Window {
    Win32Window::create_window(name, width, height)
}
