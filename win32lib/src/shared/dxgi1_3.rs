#![allow(unused_imports)]
#![allow(dead_code)]


use com::interfaces;
use cty::{ c_void };

use crate::shared::winerror::{ HRESULT };
use crate::shared::guiddef::{ REFIID };
use crate::shared::windef::{ UINT, BOOL, HWND };
use crate::shared::dxgi1_2::{ IDXGIFactory2 };

use crate::um::unknwn::{ IUnknown };

interfaces! {
    #[uuid("25483823-cd46-4c7d-86ca-47aa95b837bd")]
    pub unsafe interface IDXGIFactory3 : IDXGIFactory2 {
        pub fn GetCreationFlags(&self) -> UINT;
    }
}
