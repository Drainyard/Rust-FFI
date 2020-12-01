#![allow(unused_imports)]
#![allow(dead_code)]


use com::interfaces;
use cty::{ c_void };

use crate::shared::winerror::{ HRESULT };
use crate::shared::guiddef::{ REFIID };
use crate::shared::windef::{ UINT, BOOL, HWND };
use crate::shared::dxgi1_3::{ IDXGIFactory3 };

use crate::um::winnt::{ LUID };
use crate::um::unknwn::{ IUnknown };

interfaces! {
	#[uuid("1bc6ea02-ef36-464f-bf0c-21ca39e5168a")]
    pub unsafe interface IDXGIFactory4 : IDXGIFactory3 {
        pub fn EnumAdapterByLuid(&self, AdapterLuid: LUID, riid: REFIID, ppvAdapter: *mut *mut c_void) -> HRESULT;
        pub fn EnumWarpAdapter(&self, riid: REFIID, ppvAdapter: *mut *mut c_void) -> HRESULT;
    }
}
