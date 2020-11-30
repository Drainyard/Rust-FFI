use com::interfaces;
use cty::{ c_void };

use crate::shared::winerror::{ HRESULT };
use crate::shared::guiddef::{ REFIID };
use crate::shared::windef::{ UINT, BOOL, HWND };

use crate::um::winnt::{ LUID };
use crate::um::unknwn::{ IUnknown };



interfaces! {
    #[uuid("50c83a1c-e072-4c48-87b0-3630fa36a6d0")]
    pub unsafe interface IDXGIFactory2 : IDXGIFactory1 {
        pub fn IsWindowedStereoEnabled(&self) -> BOOL;
        pub fn CreateSwapChainForHwnd(&self, pDevice: *mut IUnknown, hWnd: HWND,
                                      pDesc: *const DXGI_SWAP_CHAIN_DESC1,
                                      pFullScreenDesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
                                      pRestrictToOutput: *mut IDXGIOutput,
                                      ppSwapChain: *mut *mut IDXGISwapChain1) -> HRESULT;
        pub fn CreateSwapChainForCoreWindow(&self, pDevice: *mut IUnknown, pWindow: *mut IUnknown,
                                            pDesc: *const DXGI_SWAP_CHAIN_DESC1,
                                            pRestrictToOutput: *mut IDXGIOutput,
                                            ppSwapChain: *mut *mut IDXGISwapChain1) -> HRESULT;
        pub fn GetSharedResourceAdapterLuid(&self, hResource: HANDLE, pLuid: *mut LUID) -> HRESULT;
        pub fn RegisterStereoStatusWindow(&self, WindowHandle: HWND, wMsg: UINT, pdwCookie: *mut DWORD) -> HRESULT;
        pub fn RegisterStereoStatusEvent(&self, hEvent: HANDLE, pdwCookie: *mut DWORD) -> HRESULT;
        pub fn UnregisterStereoStatus(&self, dwCookie: DWORD);
        pub fn RegisterOcclusionStatusWindow(&self, WindowHandle: HWND, wMsg: UINT, pdwCookie: *mut DWORD) -> HRESULT;
        pub fn RegisterOcclusionStatusEvent(&self, hEvent: HANDLE, pdwCookie: *mut DWORD) -> HRESULT;
        pub fn UnregisterOcclusionStatus(&self, dwCookie: DWORD);
        pub fn CreateSwapChainForComposition(&self, pDevice: *mut IUnknown, pDesc: *const DXGI_SWAP_CHAIN_DESC1,
                                             pRestrictToOutput: *mut IDXGIOutput,
                                             ppSwapChain: *mut *mut IDXGISwapChain1) -> HRESULT;
    }
    
    #[uuid("25483823-cd46-4c7d-86ca-47aa95b837bd")]
    pub unsafe interface IDXGIFactory3 : IDXGIFactory2 {
        pub fn GetCreationFlags(&self) -> UINT;
    }
    
    #[uuid("1bc6ea02-ef36-464f-bf0c-21ca39e5168a")]
    pub unsafe interface IDXGIFactory4 : IDXGIFactory3 {
        pub fn EnumAdapterByLuid(&self, AdapterLuid: LUID, riid: REFIID, ppvAdapter: *mut *mut c_void) -> HRESULT;

        pub fn EnumWarpAdapter(&self, riid: REFIID, ppvAdapter: *mut *mut c_void) -> HRESULT;
    }
}
