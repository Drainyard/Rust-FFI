/* typedef struct _GUID { */
/*   unsigned long  Data1; */
/*   unsigned short Data2; */
/*   unsigned short Data3; */
/*   unsigned char  Data4[8]; */
/* } GUID; */

#[repr(C)]
struct GUID {
Data1: c_ulong
}
