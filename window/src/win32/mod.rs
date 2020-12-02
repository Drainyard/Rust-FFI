#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::{ Window, BitmapBuffer };

use win32lib::shared::windef::{DWORD, HDC, HWND, GWLP_USERDATA, UINT, WPARAM, LPARAM, LRESULT, RECT,
             BOOL, CS_OWNDC, CS_VREDRAW, CS_HREDRAW};
use win32lib::um::winuser::{SetWindowLongPtrA, GetWindowLongPtrA, GetClientRect,
                            DefWindowProcA, BeginPaint, EndPaint, PostQuitMessage, GetModuleHandleA,
                            CreateWindowExA, RegisterClassA, ShowWindow, PeekMessageA,
                            GetMessageA, DispatchMessageA, TranslateMessage};
use win32lib::um::winuser::notifications::*;
use win32lib::um::winuser::{PAINTSTRUCT, WNDCLASSA, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, PM_NOREMOVE, MSG};

use win32lib::um::errhandlingapi::{GetLastError, SetLastError};

use win32lib::um::wingdi::{DeleteObject, StretchDIBits};
use win32lib::um::wingdi::{BITMAPINFO, BITMAPINFOHEADER};
use win32lib::shared::dxgi1_4::{ IDXGIFactory4 };

use std::ptr;
use cty::{c_void, c_int};
use std::ffi::{CString};
use std::convert::TryInto;

struct PixelBuffer {
    
}

#[repr(C)]
#[derive(Debug)]
struct Win32WindowState {
    bitmap_info: BITMAPINFO,
    bitmap_memory: *mut u32,

    bitmap_width: i32,
    bitmap_height: i32,
}

impl Win32WindowState {
    fn new() -> Win32WindowState {
        Win32WindowState {
            bitmap_memory: ptr::null_mut(),
            bitmap_width: 0,
            bitmap_height: 0,
            bitmap_info: Default::default(),
        }
    }
}

pub struct Win32Window {
    pub hwnd: HWND,
    window_state: Win32WindowState
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
        let error = GetLastError();
        SetLastError(error);

        // First time we call SetWindowLongPtrA the value will be 0, which means the return value will be 0 as
        // the function returns the previous value given at GWLP_USERDATA.
        if result == 0 && error != 0 {
            print_last_error(file!(), line!());
        }
    }
}

fn get_window_ptr<'a>(hwnd: HWND) -> Option<&'a mut Win32WindowState> {
    unsafe {
        let data = GetWindowLongPtrA(hwnd, GWLP_USERDATA);
        if data == 0 {
            None
        } else {
            let state = data as *mut Win32WindowState;
            if !state.is_null() {
                let data: &'a mut Win32WindowState = &mut *state;
                Some(data)
            }
            else {
                None
            }
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
            on_size(hwnd, width, height);
            
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
            
            let x = ps.rcPaint.left;
            let y = ps.rcPaint.top;
            let width = ps.rcPaint.right - ps.rcPaint.left;
            let height = ps.rcPaint.bottom - ps.rcPaint.top;

            let mut rect: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect);
            
            update_window(hwnd, hdc, rect, x, y, width, height);
            EndPaint(hwnd, &mut ps);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        WM_CLOSE => {
            PostQuitMessage(0);
            0
        },
        WM_KEYDOWN => {
            // println!("KEYDOWN: {}", w_param);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        WM_KEYUP => {
            // println!("KEYUP: {}", w_param);
            DefWindowProcA(hwnd, u_msg, w_param, l_param)
        },
        _ => DefWindowProcA(hwnd, u_msg, w_param, l_param)
    };
    // println!("0x{} {}", format!("{:01$x}", u_msg, 4), result);
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
        let bitmap_info = BITMAPINFO { bmiHeader: bitmap_info_header, bmiColors: [Default::default()]};
        window_state.bitmap_info = bitmap_info;

        if !window_state.bitmap_memory.is_null() {

            //TODO: Do something
        }
    }
}

fn update_window(hwnd: HWND, hdc: HDC, client_rect: RECT, x: c_int, y: c_int, width: c_int, height: c_int) {
    if let Some(window_state) = get_window_ptr(hwnd) {
        unsafe {
            let window_width = client_rect.right - client_rect.left;
            let window_height = client_rect.bottom - client_rect.top;

            if window_state.bitmap_memory.is_null() {
                return;
            }
            
            StretchDIBits(hdc,
                          0, 0, window_state.bitmap_width, window_state.bitmap_height,
                          0, 0, window_width, window_height,
                          window_state.bitmap_memory as *mut c_void,
                          &window_state.bitmap_info as *const BITMAPINFO,
                          DIB_RGB_COLORS, SRCCOPY
            );
        }
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
                return std::mem::zeroed();
            }
            
            let hwnd = CreateWindowExA(0, class_name.as_ptr(), name.as_ptr(), WS_OVERLAPPEDWINDOW,
                                       CW_USEDEFAULT, CW_USEDEFAULT, width as c_int, height as c_int, ptr::null_mut(),
                                       ptr::null_mut(), h_instance, ptr::null_mut());

            if hwnd.is_null() {
                print_last_error(file!(), line!());
                return std::mem::zeroed();
            }

            // 5 for SW_SHOW
            let show = ShowWindow(hwnd, 5);
            let window_state = Win32WindowState::new();

            let window = Win32Window { hwnd, window_state};

            window
        }
    }

    fn update_buffer(&mut self, buffer: &mut [u32], width: i32, height: i32) {
        set_window_ptr(self.hwnd, &self.window_state);
        self.window_state.bitmap_memory = buffer.as_mut_ptr();
        self.window_state.bitmap_width = width;
        self.window_state.bitmap_height = height;
    }

    fn get_bitmap(&self) -> BitmapBuffer {
        let size: usize = (self.window_state.bitmap_width * self.window_state.bitmap_height * 4).try_into().unwrap();
        let bitmap_as_slice = unsafe {
            std::slice::from_raw_parts_mut(self.window_state.bitmap_memory, size)
        };
        println!("{} {}", self.window_state.bitmap_width, self.window_state.bitmap_height);
        
        set_window_ptr(self.hwnd, &self.window_state);
        
        BitmapBuffer {width: self.window_state.bitmap_width, height: self.window_state.bitmap_height,
        bitmap: bitmap_as_slice}
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
