use com::interfaces;
use cty::{ c_void };

use crate::shared::guiddef::{ REFIID };
use crate::shared::winerror::{ HRESULT };
use crate::shared::minwindef::{ ULONG };

interfaces! {
    #[uuid("00000000-0000-0000-C000-000000000046")]
    pub unsafe interface IUnknown {
        pub fn QueryInterface(&self, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT;
        pub fn AddRef(&self) -> ULONG;
        pub fn Release(&self) -> ULONG;
    }
}
