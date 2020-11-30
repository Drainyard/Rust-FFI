use cty::{ c_ulong, c_ushort, c_uchar };

#[repr(C)]
struct GUID {
    Data1: c_ulong,
    Data2: c_ushort,
    Data3: c_ushort,
    Data4: [c_uchar; 8]
}

type IID = GUID;
type REFIID = *const IID;
