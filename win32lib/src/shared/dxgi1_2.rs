use com::interfaces;
use cty::{ c_void };

use crate::shared::winerror::{ HRESULT };
use crate::shared::guiddef::{ REFIID };
use crate::shared::windef::{ UINT, BOOL, HWND, RECT, POINT, HANDLE, DWORD };
use crate::shared::dxgiformat::{ DXGI_FORMAT };
use crate::shared::dxgicommon::{ DXGI_SAMPLE_DESC, DXGI_RATIONAL };
use crate::shared::dxgitype::{ DXGI_MODE_SCANLINE_ORDER, DXGI_MODE_SCALING, DXGI_RGBA, DXGI_MODE_ROTATION };
use crate::shared::dxgi::{ DXGI_USAGE, DXGI_SWAP_EFFECT, IDXGIOutput, IDXGISwapChain, IDXGIFactory1, LUID };

use crate::um::unknwn::{ IUnknown };

ENUM!{enum DXGI_SCALING {
    DXGI_SCALING_STRETCH              = 0,
    DXGI_SCALING_NONE                 = 1,
    DXGI_SCALING_ASPECT_RATIO_STRETCH = 2,
}}

ENUM!{enum DXGI_ALPHA_MODE {
    DXGI_ALPHA_MODE_UNSPECIFIED    = 0,
    DXGI_ALPHA_MODE_PREMULTIPLIED  = 1,
    DXGI_ALPHA_MODE_STRAIGHT       = 2,
    DXGI_ALPHA_MODE_IGNORE         = 3,
    DXGI_ALPHA_MODE_FORCE_DWORD    = 4,
}}

STRUCT! { struct DXGI_SWAP_CHAIN_DESC1 {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    Stereo: BOOL,
    SampleDesc: DXGI_SAMPLE_DESC,
    BufferUsage: DXGI_USAGE,
    BufferCount: UINT,
    Scaling: DXGI_SCALING,
    SwapEffect: DXGI_SWAP_EFFECT,
    AlphaMode: DXGI_ALPHA_MODE,
    Flags: UINT,
}}


STRUCT! { struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    RefreshRate: DXGI_RATIONAL,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
    Windowed: BOOL,
}}

STRUCT! { struct DXGI_PRESENT_PARAMETERS {
    DirtyRectsCount: UINT,
    pDirtyRects: *mut RECT,
    pScrollRect: *mut RECT,
    pScrollOffset: *mut POINT,
}}

interfaces! {       
    #[uuid("790a45f7-0d42-4876-983a-0a55cfe6f4aa")]
    pub unsafe interface IDXGISwapChain1 : IDXGISwapChain {
        pub fn GetDesc1(&self, pDesc: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT;
        pub fn GetFullscreenDesc(&self, pDesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> HRESULT;
        pub fn GetHwnd(&self, pHwnd: *mut HWND) -> HRESULT;
        pub fn GetCoreWindow(&self, refiid: REFIID, ppUnk: *mut *mut c_void) -> HRESULT;
        pub fn Present1(&self, SyncInterval: UINT, PresentFlags: UINT,
                        pPresentParameters: *const DXGI_PRESENT_PARAMETERS) -> HRESULT;
        pub fn IsTemporaryMonoSupported(&self) -> BOOL;
        pub fn GetRestrictToOutput(&self, ppRestrictToOutput: *mut IDXGIOutput) -> HRESULT;
        pub fn SetBackgroundColor(&self, pColor: *const DXGI_RGBA) -> HRESULT;
        pub fn GetBackgroundColor(&self, pColor: *mut DXGI_RGBA) -> HRESULT;
        pub fn SetRotation(&self, Rotation: DXGI_MODE_ROTATION) -> HRESULT;
        pub fn GetRotation(&self, pRotation: *mut DXGI_MODE_ROTATION) -> HRESULT;
    }
    
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
}

DEFINE_GUID!{IID_IDXGISwapChain1, 0x790a45f7, 0x0d42, 0x4876, 0x98, 0x3a, 0x0a, 0x55, 0xcf, 0xe6, 0xf4, 0xaa}
DEFINE_GUID!{IID_IDXGIFactory2, 0x50c83a1c, 0xe072, 0x4c48, 0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0}
