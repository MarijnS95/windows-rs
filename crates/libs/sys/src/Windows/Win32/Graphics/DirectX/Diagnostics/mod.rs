pub const CLSID_DxDiagProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa65b8071_3bfe_4213_9a5b_491da4461ca7);
pub const DXDIAG_DX9_SDK_VERSION: u32 = 111u32;
pub const DXDIAG_E_INSUFFICIENT_BUFFER: windows_sys::core::HRESULT = 0x8007007A_u32 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _DXDIAG_INIT_PARAMS {
    pub dwSize: u32,
    pub dwDxDiagHeaderVersion: u32,
    pub bAllowWHQLChecks: windows_sys::core::BOOL,
    pub pReserved: *mut core::ffi::c_void,
}
