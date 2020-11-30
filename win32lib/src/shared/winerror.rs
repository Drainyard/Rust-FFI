pub const S_OK: LONG           = 0x00000000; 	// Operation successful
pub const E_NOTIMPL: LONG      = 0x80004001; 	// Not implemented
pub const E_NOLONGERFACE: LONG  = 0x80004002; 	// No such interface supported
pub const E_POLONGER: LONG      = 0x80004003; 	// Pointer that is not valid
pub const E_ABORT: LONG        = 0x80004004; 	// Operation aborted
pub const E_FAIL: LONG         = 0x80004005; 	// Unspecified failure
pub const E_UNEXPECTED: LONG   = 0x8000FFFF; 	// Unexpected failure
pub const E_ACCESSDENIED: LONG = 0x80070005; 	// General access denied error
pub const E_HANDLE: LONG       = 0x80070006; 	// Handle that is not valid
pub const E_OUTOFMEMORY: LONG  = 0x8007000E; 	// Failed to allocate necessary memory
pub const E_INVALIDARG: LONG   = 0x80070057; 	// One or more arguments are not valid

type HRESULT = LONG;
