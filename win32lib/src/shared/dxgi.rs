use com::interfaces;
use cty::{ c_void };

use crate::shared::winerror::{ HRESULT };
use crate::shared::guiddef::{ REFIID, REFGUID };
use crate::shared::windef::{ UINT, BOOL, HWND, RECT, HMONITOR, BYTE, HMODULE };
use crate::shared::basetsd::{ SIZE_T };

use crate::um::winnt::{ LUID, WCHAR, LARGE_INTEGER, INT };
use crate::um::unknwn::{ IUnknown };

use crate::shared::dxgitype::{ DXGI_MODE_ROTATION, DXGI_MODE_DESC, DXGI_GAMMA_CONTROL,
                               DXGI_GAMMA_CONTROL_CAPABILITIES };
use crate::shared::dxgiformat::{ DXGI_FORMAT };
use crate::shared::dxgicommon::{ DXGI_SAMPLE_DESC };

ENUM!{enum DXGI_USAGE {
    DXGI_CPU_ACCESS_NONE             = 0,
    DXGI_CPU_ACCESS_DYNAMIC          = 1,
    DXGI_CPU_ACCESS_READ_WRITE       = 2,
    DXGI_CPU_ACCESS_SCRATCH          = 3,
    DXGI_CPU_ACCESS_FIELD            = 15,
    DXGI_USAGE_SHADER_INPUT          = 1 << (0 + 4), 
    DXGI_USAGE_RENDER_TARGET_OUTPUT  = 1 << (1 + 4), 
    DXGI_USAGE_BACK_BUFFER           = 1 << (2 + 4), 
    DXGI_USAGE_SHARED                = 1 << (3 + 4), 
    DXGI_USAGE_READ_ONLY             = 1 << (4 + 4), 
    DXGI_USAGE_DISCARD_ON_PRESENT    = 1 << (5 + 4), 
    DXGI_USAGE_UNORDERED_ACCESS      = 1 << (6 + 4),
}}

ENUM!{enum DXGI_SWAP_EFFECT {
    DXGI_SWAP_EFFECT_DISCARD          = 0,
    DXGI_SWAP_EFFECT_SEQUENTIAL       = 1,
    DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL  = 2,
    DXGI_SWAP_EFFECT_FLIP_DISCARD     = 3,
}}

STRUCT! { struct DXGI_OUTPUT_DESC {
    DeviceName: [WCHAR; 32],
    DesktopCoordinates: RECT,
    AttachedToDesktop: BOOL,
    Rotation: DXGI_MODE_ROTATION,
    Monitor: HMONITOR,
}}

STRUCT! { struct DXGI_SURFACE_DESC {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    SampleDesc: DXGI_SAMPLE_DESC,
}}

STRUCT! {struct DXGI_MAPPED_RECT {
    Pitch: INT,
    pBits: *mut BYTE,
}}

STRUCT! { struct DXGI_FRAME_STATISTICS {
    PresentCount: UINT,
    PresentRefreshCount: UINT,
    SyncRefreshCount: UINT,
    SyncQPCTime: LARGE_INTEGER,
    SyncGPUTime: LARGE_INTEGER,
}}

STRUCT! { struct DXGI_SWAP_CHAIN_DESC {
    BufferDesc: DXGI_MODE_DESC,
    SampleDesc: DXGI_SAMPLE_DESC,
    BufferUsage: DXGI_USAGE,
    BufferCount: UINT,
    OutputWindow: HWND,
    Windowed: BOOL,
    SwapEffect: DXGI_SWAP_EFFECT,
    Flags: UINT,
}}

STRUCT! { struct DXGI_ADAPTER_DESC {
    Description: [WCHAR; 128],
    VendorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
}}

STRUCT! { struct DXGI_ADAPTER_DESC1 {
    Description: [WCHAR; 128],
    VendorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
    Flags: UINT,
}}

interfaces! {
    #[uuid("29038f61-3839-4626-91fd-086879011a05")]
    pub unsafe interface IDXGIAdapter1 : IDXGIAdapter {
        pub fn GetDesc1(&self, pDesc: *mut DXGI_ADAPTER_DESC1) -> HRESULT;
    }
    
    #[uuid("770aae78-f26f-4dba-a829-253c83d1b387")]
    pub unsafe interface IDXGIFactory1 : IDXGIFactory {
        pub fn EnumAdapters1(&self, Adapter: UINT, ppAdapter: *mut *mut IDXGIAdapter1) -> HRESULT;
        pub fn IsCurrent(&self) -> BOOL;
    }
    
    #[uuid("2411e7e1-12ac-4ccf-bd14-9798e8534dc0")]
    pub unsafe interface IDXGIAdapter : IDXGIOutput {
        pub fn EnumOutputs(&self, Output: UINT, ppOutput: *mut IDXGIOutput) -> HRESULT;
        pub fn GetDesc(&self, pDesc: *mut DXGI_ADAPTER_DESC) -> HRESULT;
        pub fn CheckInterfaceSupport(&self, InterfaceName: REFGUID, pUMDVersion: *mut LARGE_INTEGER);
    }
    
    #[uuid("7b7166ec-21c7-44ae-b21a-c9ae321ae369")]
    pub unsafe interface IDXGIFactory : IDXGIObject {
        pub fn EnumAdapters(&self, Adapter: UINT, ppAdapter: *mut *mut IDXGIAdapter) -> HRESULT;
        pub fn MakeWindowAssociation(&self, WindowHandle: HWND, Flags: UINT) -> HRESULT;
        pub fn GetWindowAssociation(&self, pWindowHandle: *mut HWND) -> HRESULT;
        pub fn CreateSwapChain(&self, pDevice: *mut IUnknown, pDesc: *mut DXGI_SWAP_CHAIN_DESC,
                               ppSwapChain: *mut *mut IDXGISwapChain) -> HRESULT;
        pub fn CreateSoftwareAdapter(&self, Module: HMODULE, ppAdapter: *mut *mut IDXGIAdapter) -> HRESULT;
    }
    
    #[uuid("310d36a0-d2e7-4c0a-aa04-6a9d23b8886a")]
    pub unsafe interface IDXGISwapChain : IDXGIDeviceSubObject {
        pub fn Present(&self, SyncInterval: UINT, Flags: UINT) -> HRESULT;
        pub fn GetBuffer(&self, Buffer: UINT, riid: REFIID, ppSurface: *mut c_void) -> HRESULT;
        pub fn SetFullscreenState(&self, Fullscreen: BOOL, pTarget: *mut IDXGIOutput) -> HRESULT;
        pub fn GetFullscreenState(&self, pFullscreen: *mut BOOL, ppTarget: *mut *mut IDXGIOutput) -> HRESULT;
        pub fn GetDesc(&self, pDesc: *mut DXGI_SWAP_CHAIN_DESC) -> HRESULT;
        pub fn ResizeBuffers(&self, BufferCount: UINT, Width: UINT, Height: UINT, NewFormat: DXGI_FORMAT,
                             SwapChainFlags: UINT) -> HRESULT;
        pub fn ResizeTarget(&self, pNewTargetParameters: *const DXGI_MODE_DESC) -> HRESULT;
        pub fn GetContainingOutput(&self, ppOutput: *mut *mut IDXGIOutput) -> HRESULT;
        pub fn GetFrameStatistics(&self, pStats: *mut DXGI_FRAME_STATISTICS) -> HRESULT;
        pub fn GetLastPresentCount(&self, pLastPresentCount: *mut UINT) -> HRESULT;
    }
    
    #[uuid("aec22fb8-76f3-4639-9be0-28eb43a67a2e")]
    pub unsafe interface IDXGIObject : IUnknown {

        pub fn SetPrivateData(&self, Name: REFGUID,
                              DataSize: UINT,
                              pData: *const c_void) -> HRESULT;
        pub fn SetPrivateDataInterface(&self, Name: REFGUID,
                                       pUnknown: *const IUnknown) -> HRESULT;
        pub fn GetPrivateData(&self, Name: REFGUID,
                              pDataSize: *mut UINT,
                              pData: *mut c_void) -> HRESULT;
        pub fn GetParent(&self, riid: REFIID,
                         ppParent: *mut *mut c_void) -> HRESULT;
        
    }
    
    #[uuid("3d3e0379-f9de-4d58-bb6c-18d62992f1a6")]
    pub unsafe interface IDXGIDeviceSubObject : IDXGIObject {
        pub fn GetDevice(&self, riid: REFIID, ppDevice: *mut *mut c_void) -> HRESULT;
    }
    
    #[uuid("cafcb56c-6ac3-4889-bf47-9e23bbd260ec")]
    pub unsafe interface IDXGISurface : IDXGIDeviceSubObject {
        pub fn GetDesc(&self, pDesc: *mut DXGI_SURFACE_DESC) ->  HRESULT;
        pub fn Map(&self, pLockedRect: *mut DXGI_MAPPED_RECT, MapFlags: UINT) -> HRESULT;
        pub fn Unmap(&self) -> HRESULT;
    }
    
    #[uuid("ae02eedb-c735-4690-8d52-5a8dc20213aa")]
    pub unsafe interface IDXGIOutput : IDXGIObject {
        pub fn GetDesc(&self, pDesc: *mut DXGI_OUTPUT_DESC) -> HRESULT;
        pub fn GetDisplayModeList(&self, EnumFormat: DXGI_FORMAT, Flags: UINT, pNumModes: *mut UINT,
                                  pDesc: *mut DXGI_MODE_DESC) -> HRESULT;
        pub fn FindClosestMatchingMode(&self, pModeToMatch: *const DXGI_MODE_DESC,
                                       pClosestMatch: *mut DXGI_MODE_DESC,
                                       pConcernedDevice: *mut IUnknown) -> HRESULT;
        pub fn WaitForVBlank(&self) -> HRESULT;
        pub fn TakeOwnership(&self, pDevice: *mut IUnknown, Exclusive: BOOL) -> HRESULT;
        pub fn ReleaseOwnership(&self);
        pub fn GetGammaControlCapabilities(&self, pGammaCaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> HRESULT;
        pub fn SetGammaControl(&self, pArray: *const DXGI_GAMMA_CONTROL) -> HRESULT;
        pub fn GetGammaControl(&self, pArray: *mut DXGI_GAMMA_CONTROL) -> HRESULT;
        pub fn SetDisplaeSurface(&self, pScanoutSurface: *mut IDXGISurface) -> HRESULT;
        pub fn GetDisplaySurface(&self, pDestination: *mut IDXGISurface) -> HRESULT;
        pub fn GetFrameStatistics(&self, pStats: *mut DXGI_FRAME_STATISTICS)  -> HRESULT;
    }
}
