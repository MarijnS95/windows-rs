pub const CLSID_DxDiagProvider: windows_core::GUID = windows_core::GUID::from_u128(0xa65b8071_3bfe_4213_9a5b_491da4461ca7);
pub const DXDIAG_DX9_SDK_VERSION: u32 = 111u32;
pub const DXDIAG_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x8007007A_u32 as _);
windows_core::imp::define_interface!(IDxDiagContainer, IDxDiagContainer_Vtbl, 0x7d0f462f_4064_4862_bc7f_933e5058c10f);
windows_core::imp::interface_hierarchy!(IDxDiagContainer, windows_core::IUnknown);
impl IDxDiagContainer {
    pub unsafe fn GetNumberOfChildContainers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfChildContainers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumChildContainerNames(&self, dwindex: u32, pwszcontainer: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumChildContainerNames)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwszcontainer.as_ptr()), pwszcontainer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetChildContainer<P0>(&self, pwszcontainer: P0) -> windows_core::Result<IDxDiagContainer>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildContainer)(windows_core::Interface::as_raw(self), pwszcontainer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNumberOfProps(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfProps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumPropNames(&self, dwindex: u32, pwszpropname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumPropNames)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwszpropname.as_ptr()), pwszpropname.len().try_into().unwrap()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProp<P0>(&self, pwszpropname: P0) -> windows_core::Result<super::super::super::System::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProp)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
pub struct IDxDiagContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfChildContainers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumChildContainerNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetChildContainer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumberOfProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumPropNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProp: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDxDiagContainer_Impl: windows_core::IUnknownImpl {
    fn GetNumberOfChildContainers(&self) -> windows_core::Result<u32>;
    fn EnumChildContainerNames(&self, dwindex: u32, pwszcontainer: windows_core::PWSTR, cchcontainer: u32) -> windows_core::Result<()>;
    fn GetChildContainer(&self, pwszcontainer: &windows_core::PCWSTR) -> windows_core::Result<IDxDiagContainer>;
    fn GetNumberOfProps(&self) -> windows_core::Result<u32>;
    fn EnumPropNames(&self, dwindex: u32, pwszpropname: windows_core::PWSTR, cchpropname: u32) -> windows_core::Result<()>;
    fn GetProp(&self, pwszpropname: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDxDiagContainer_Vtbl {
    pub const fn new<Identity: IDxDiagContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberOfChildContainers<Identity: IDxDiagContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxDiagContainer_Impl::GetNumberOfChildContainers(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumChildContainerNames<Identity: IDxDiagContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwszcontainer: windows_core::PWSTR, cchcontainer: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxDiagContainer_Impl::EnumChildContainerNames(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwszcontainer), core::mem::transmute_copy(&cchcontainer)).into()
            }
        }
        unsafe extern "system" fn GetChildContainer<Identity: IDxDiagContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcontainer: windows_core::PCWSTR, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxDiagContainer_Impl::GetChildContainer(this, core::mem::transmute(&pwszcontainer)) {
                    Ok(ok__) => {
                        ppinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfProps<Identity: IDxDiagContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxDiagContainer_Impl::GetNumberOfProps(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPropNames<Identity: IDxDiagContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwszpropname: windows_core::PWSTR, cchpropname: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxDiagContainer_Impl::EnumPropNames(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwszpropname), core::mem::transmute_copy(&cchpropname)).into()
            }
        }
        unsafe extern "system" fn GetProp<Identity: IDxDiagContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvarprop: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxDiagContainer_Impl::GetProp(this, core::mem::transmute(&pwszpropname)) {
                    Ok(ok__) => {
                        pvarprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfChildContainers: GetNumberOfChildContainers::<Identity, OFFSET>,
            EnumChildContainerNames: EnumChildContainerNames::<Identity, OFFSET>,
            GetChildContainer: GetChildContainer::<Identity, OFFSET>,
            GetNumberOfProps: GetNumberOfProps::<Identity, OFFSET>,
            EnumPropNames: EnumPropNames::<Identity, OFFSET>,
            GetProp: GetProp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxDiagContainer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDxDiagContainer {}
windows_core::imp::define_interface!(IDxDiagProvider, IDxDiagProvider_Vtbl, 0x9c6b4cb0_23f8_49cc_a3ed_45a55000a6d2);
windows_core::imp::interface_hierarchy!(IDxDiagProvider, windows_core::IUnknown);
impl IDxDiagProvider {
    pub unsafe fn Initialize(&self, pparams: *const _DXDIAG_INIT_PARAMS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pparams).ok() }
    }
    pub unsafe fn GetRootContainer(&self) -> windows_core::Result<IDxDiagContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDxDiagProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const _DXDIAG_INIT_PARAMS) -> windows_core::HRESULT,
    pub GetRootContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxDiagProvider_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pparams: *const _DXDIAG_INIT_PARAMS) -> windows_core::Result<()>;
    fn GetRootContainer(&self) -> windows_core::Result<IDxDiagContainer>;
}
impl IDxDiagProvider_Vtbl {
    pub const fn new<Identity: IDxDiagProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDxDiagProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const _DXDIAG_INIT_PARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxDiagProvider_Impl::Initialize(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        unsafe extern "system" fn GetRootContainer<Identity: IDxDiagProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxDiagProvider_Impl::GetRootContainer(this) {
                    Ok(ok__) => {
                        ppinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetRootContainer: GetRootContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxDiagProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxDiagProvider {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _DXDIAG_INIT_PARAMS {
    pub dwSize: u32,
    pub dwDxDiagHeaderVersion: u32,
    pub bAllowWHQLChecks: windows_core::BOOL,
    pub pReserved: *mut core::ffi::c_void,
}
impl Default for _DXDIAG_INIT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
