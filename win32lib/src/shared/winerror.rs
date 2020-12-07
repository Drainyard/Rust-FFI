use cty::{c_ulong};

pub const S_OK: HRESULT           = 0x00000000; 	// Operation successful
pub const E_NOTIMPL: HRESULT      = 0x80004001; 	// Not implemented
pub const E_NOHRESULTERFACE: HRESULT  = 0x80004002; 	// No such interface supported
pub const E_POHRESULTER: HRESULT      = 0x80004003; 	// Pointer that is not valid
pub const E_ABORT: HRESULT        = 0x80004004; 	// Operation aborted
pub const E_FAIL: HRESULT         = 0x80004005; 	// Unspecified failure
pub const E_UNEXPECTED: HRESULT   = 0x8000FFFF; 	// Unexpected failure
pub const E_ACCESSDENIED: HRESULT = 0x80070005; 	// General access denied error
pub const E_HANDLE: HRESULT       = 0x80070006; 	// Handle that is not valid
pub const E_OUTOFMEMORY: HRESULT  = 0x8007000E; 	// Failed to allocate necessary memory
pub const E_INVALIDARG: HRESULT   = 0x80070057; 	// One or more arguments are not valid

pub const DXGI_ERROR_NOT_FOUND: HRESULT = 0x887A0002;

pub type HRESULT = c_ulong;

#[inline]
pub fn SUCCEEDED(hr: HRESULT) -> bool {
    hr >= 0
}
