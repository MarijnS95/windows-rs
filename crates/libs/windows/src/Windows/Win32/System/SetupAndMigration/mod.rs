#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn OOBEComplete(isoobecomplete : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    unsafe { OOBEComplete(core::mem::transmute(isoobecomplete)).ok() }
}
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>, waithandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn RegisterWaitUntilOOBECompleted(oobecompletedcallback : OOBE_COMPLETED_CALLBACK, callbackcontext : *const core::ffi::c_void, waithandle : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    unsafe { RegisterWaitUntilOOBECompleted(oobecompletedcallback, core::mem::transmute(callbackcontext.unwrap_or(core::mem::zeroed())), core::mem::transmute(waithandle)).ok() }
}
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UnregisterWaitUntilOOBECompleted(waithandle : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    unsafe { UnregisterWaitUntilOOBECompleted(waithandle).ok() }
}
pub type OOBE_COMPLETED_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void)>;
