use cty::c_void;

use crate::um::unknwn;
use crate::um::d3dcommon;
use crate::shared::winerror;
use crate::shared::guiddef;


extern "system" {
    pub fn D3D12CreateDevice(pAdapter: *mut unknwn::IUnknown, MinimumFeatureLevel: d3dcommon::D3D_FEATURE_LEVEL,
                             riid: guiddef::REFIID, ppDevice: *mut *mut c_void) -> winerror::HRESULT;
}
