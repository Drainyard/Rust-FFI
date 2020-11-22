use crate::shared::windef::{DWORD};

extern "system" {
    pub fn GetLastError() -> DWORD;
    pub fn SetLastError(dwErrCode: DWORD);
}
