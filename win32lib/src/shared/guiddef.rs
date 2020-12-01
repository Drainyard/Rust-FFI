use cty::{ c_ulong, c_ushort, c_uchar };

STRUCT! { struct GUID {
    Data1: c_ulong,
    Data2: c_ushort,
    Data3: c_ushort,
    Data4: [c_uchar; 8],
}}

pub type IID = GUID;
pub type REFIID = *const IID;
pub type REFGUID = *const GUID;
