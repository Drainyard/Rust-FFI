
#[cfg(target_os ="windows")]
use window::win32::create_window;
use win32lib::shared::winerror;
use win32lib::shared::dxgi;
use win32lib::shared::dxgi1_2;
use win32lib::shared::dxgi1_3;
use win32lib::um::d3d12;
use win32lib::um::d3dcommon;
use window::Window;

fn main() {
    let window = create_window("Test Window", 800, 600);

    let mut factory: Option<dxgi1_2::IDXGIFactory2> = None;
    let result = unsafe {
        dxgi1_3::CreateDXGIFactory2(0, &dxgi1_2::IID_IDXGIFactory2 as *const _ as _,
                           &mut factory as *mut _ as _)
    };

    if result != 0 {
        panic!("Error when creating factory: 0x{:x}", result);
    }

    let factory = factory.unwrap();
    let mut adapter_index = 0;
    'adapters: loop {
        let mut adapter: *mut dxgi::IDXGIAdapter1 = std::ptr::null_mut();
        let err = unsafe {
            factory.EnumAdapters1(adapter_index, &mut adapter as *mut *mut _ as *mut *mut _)
        };
        
        if err == winerror::DXGI_ERROR_NOT_FOUND {
            break 'adapters;
        }

        let mut desc: dxgi::DXGI_ADAPTER_DESC1 = unsafe { std::mem::zeroed() };
        unsafe {
            (*adapter).GetDesc1(&mut desc);
        }

        if desc.Flags & dxgi::DXGI_ADAPTER_FLAG_SOFTWARE != 0 {
            continue;
        }

        unsafe {
            d3d12::D3D12CreateDevice(&mut adapter, d3dcommon::D3D_FEATURE_LEVEL_12_0, ID3D12Device::uuidof(), nullptr);
        }

        adapter_index = adapter_index + 1;
    }

    // for (UINT adapterIndex = 0; DXGI_ERROR_NOT_FOUND != factory->EnumAdapters1(adapterIndex, &adapter); ++adapterIndex)
    // {

    
    while window.is_open() {
        window.poll_events();
    }
}
