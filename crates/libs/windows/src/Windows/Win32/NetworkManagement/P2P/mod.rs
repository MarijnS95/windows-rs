#[inline]
pub unsafe fn DrtClose(hdrt: *const core::ffi::c_void) {
    windows_link::link!("drt.dll" "system" fn DrtClose(hdrt : *const core::ffi::c_void));
    unsafe { DrtClose(hdrt) }
}
#[inline]
pub unsafe fn DrtContinueSearch(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtContinueSearch(hsearchcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtContinueSearch(hsearchcontext).ok() }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<DRT_DATA> {
    windows_link::link!("drtprov.dll" "system" fn DrtCreateDerivedKey(plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, pkey : *mut DRT_DATA) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtCreateDerivedKey(plocalcert, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: Option<*const super::super::Security::Cryptography::CERT_CONTEXT>) -> windows_core::Result<*mut DRT_SECURITY_PROVIDER> {
    windows_link::link!("drtprov.dll" "system" fn DrtCreateDerivedKeySecurityProvider(prootcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtCreateDerivedKeySecurityProvider(prootcert, plocalcert.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtCreateDnsBootstrapResolver<P1>(port: u16, pwszaddress: P1) -> windows_core::Result<*mut DRT_BOOTSTRAP_PROVIDER>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("drtprov.dll" "system" fn DrtCreateDnsBootstrapResolver(port : u16, pwszaddress : windows_core::PCWSTR, ppmodule : *mut *mut DRT_BOOTSTRAP_PROVIDER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtCreateDnsBootstrapResolver(port, pwszaddress.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drttransport.dll" "system" fn DrtCreateIpv6UdpTransport(scope : DRT_SCOPE, dwscopeid : u32, dwlocalitythreshold : u32, pwport : *mut u16, phtransport : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtCreateIpv6UdpTransport(scope, dwscopeid, dwlocalitythreshold, pwport as _, phtransport as _).ok() }
}
#[inline]
pub unsafe fn DrtCreateNullSecurityProvider() -> windows_core::Result<*mut DRT_SECURITY_PROVIDER> {
    windows_link::link!("drtprov.dll" "system" fn DrtCreateNullSecurityProvider(ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtCreateNullSecurityProvider(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtCreatePnrpBootstrapResolver<P1, P2, P3>(fpublish: bool, pwzpeername: P1, pwzcloudname: P2, pwzpublishingidentity: P3) -> windows_core::Result<*mut DRT_BOOTSTRAP_PROVIDER>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("drtprov.dll" "system" fn DrtCreatePnrpBootstrapResolver(fpublish : windows_core::BOOL, pwzpeername : windows_core::PCWSTR, pwzcloudname : windows_core::PCWSTR, pwzpublishingidentity : windows_core::PCWSTR, ppresolver : *mut *mut DRT_BOOTSTRAP_PROVIDER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtCreatePnrpBootstrapResolver(fpublish.into(), pwzpeername.param().abi(), pwzcloudname.param().abi(), pwzpublishingidentity.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    windows_link::link!("drtprov.dll" "system" fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider : *const DRT_SECURITY_PROVIDER));
    unsafe { DrtDeleteDerivedKeySecurityProvider(psecurityprovider) }
}
#[inline]
pub unsafe fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    windows_link::link!("drtprov.dll" "system" fn DrtDeleteDnsBootstrapResolver(presolver : *const DRT_BOOTSTRAP_PROVIDER));
    unsafe { DrtDeleteDnsBootstrapResolver(presolver) }
}
#[inline]
pub unsafe fn DrtDeleteIpv6UdpTransport(htransport: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drttransport.dll" "system" fn DrtDeleteIpv6UdpTransport(htransport : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtDeleteIpv6UdpTransport(htransport).ok() }
}
#[inline]
pub unsafe fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    windows_link::link!("drtprov.dll" "system" fn DrtDeleteNullSecurityProvider(psecurityprovider : *const DRT_SECURITY_PROVIDER));
    unsafe { DrtDeleteNullSecurityProvider(psecurityprovider) }
}
#[inline]
pub unsafe fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    windows_link::link!("drtprov.dll" "system" fn DrtDeletePnrpBootstrapResolver(presolver : *const DRT_BOOTSTRAP_PROVIDER));
    unsafe { DrtDeletePnrpBootstrapResolver(presolver) }
}
#[inline]
pub unsafe fn DrtEndSearch(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtEndSearch(hsearchcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtEndSearch(hsearchcontext).ok() }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DrtGetEventData(hdrt: *const core::ffi::c_void, uleventdatalen: u32, peventdata: *mut DRT_EVENT_DATA) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtGetEventData(hdrt : *const core::ffi::c_void, uleventdatalen : u32, peventdata : *mut DRT_EVENT_DATA) -> windows_core::HRESULT);
    unsafe { DrtGetEventData(hdrt, uleventdatalen, peventdata as _).ok() }
}
#[inline]
pub unsafe fn DrtGetEventDataSize(hdrt: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_link::link!("drt.dll" "system" fn DrtGetEventDataSize(hdrt : *const core::ffi::c_void, puleventdatalen : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtGetEventDataSize(hdrt, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtGetInstanceName(hdrt: *const core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtGetInstanceName(hdrt : *const core::ffi::c_void, ulcbinstancenamesize : u32, pwzdrtinstancename : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DrtGetInstanceName(hdrt, ulcbinstancenamesize, core::mem::transmute(pwzdrtinstancename)).ok() }
}
#[inline]
pub unsafe fn DrtGetInstanceNameSize(hdrt: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_link::link!("drt.dll" "system" fn DrtGetInstanceNameSize(hdrt : *const core::ffi::c_void, pulcbinstancenamesize : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtGetInstanceNameSize(hdrt, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DrtGetSearchPath(hsearchcontext: *const core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtGetSearchPath(hsearchcontext : *const core::ffi::c_void, ulsearchpathsize : u32, psearchpath : *mut DRT_ADDRESS_LIST) -> windows_core::HRESULT);
    unsafe { DrtGetSearchPath(hsearchcontext, ulsearchpathsize, psearchpath as _).ok() }
}
#[inline]
pub unsafe fn DrtGetSearchPathSize(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_link::link!("drt.dll" "system" fn DrtGetSearchPathSize(hsearchcontext : *const core::ffi::c_void, pulsearchpathsize : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtGetSearchPathSize(hsearchcontext, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtGetSearchResult(hsearchcontext: *const core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtGetSearchResult(hsearchcontext : *const core::ffi::c_void, ulsearchresultsize : u32, psearchresult : *mut DRT_SEARCH_RESULT) -> windows_core::HRESULT);
    unsafe { DrtGetSearchResult(hsearchcontext, ulsearchresultsize, psearchresult as _).ok() }
}
#[inline]
pub unsafe fn DrtGetSearchResultSize(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_link::link!("drt.dll" "system" fn DrtGetSearchResultSize(hsearchcontext : *const core::ffi::c_void, pulsearchresultsize : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DrtGetSearchResultSize(hsearchcontext, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DrtOpen(psettings: *const DRT_SETTINGS, hevent: Option<super::super::Foundation::HANDLE>, pvcontext: Option<*const core::ffi::c_void>, phdrt: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtOpen(psettings : *const DRT_SETTINGS, hevent : super::super::Foundation:: HANDLE, pvcontext : *const core::ffi::c_void, phdrt : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtOpen(psettings, hevent.unwrap_or(core::mem::zeroed()) as _, pvcontext.unwrap_or(core::mem::zeroed()) as _, phdrt as _).ok() }
}
#[inline]
pub unsafe fn DrtRegisterKey(hdrt: *const core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: Option<*const core::ffi::c_void>, phkeyregistration: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtRegisterKey(hdrt : *const core::ffi::c_void, pregistration : *const DRT_REGISTRATION, pvkeycontext : *const core::ffi::c_void, phkeyregistration : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtRegisterKey(hdrt, pregistration, pvkeycontext.unwrap_or(core::mem::zeroed()) as _, phkeyregistration as _).ok() }
}
#[inline]
pub unsafe fn DrtStartSearch(hdrt: *const core::ffi::c_void, pkey: *const DRT_DATA, pinfo: Option<*const DRT_SEARCH_INFO>, timeout: u32, hevent: super::super::Foundation::HANDLE, pvcontext: Option<*const core::ffi::c_void>, hsearchcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtStartSearch(hdrt : *const core::ffi::c_void, pkey : *const DRT_DATA, pinfo : *const DRT_SEARCH_INFO, timeout : u32, hevent : super::super::Foundation:: HANDLE, pvcontext : *const core::ffi::c_void, hsearchcontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DrtStartSearch(hdrt, pkey, pinfo.unwrap_or(core::mem::zeroed()) as _, timeout, hevent, pvcontext.unwrap_or(core::mem::zeroed()) as _, hsearchcontext as _).ok() }
}
#[inline]
pub unsafe fn DrtUnregisterKey(hkeyregistration: *const core::ffi::c_void) {
    windows_link::link!("drt.dll" "system" fn DrtUnregisterKey(hkeyregistration : *const core::ffi::c_void));
    unsafe { DrtUnregisterKey(hkeyregistration) }
}
#[inline]
pub unsafe fn DrtUpdateKey(hkeyregistration: *const core::ffi::c_void, pappdata: *const DRT_DATA) -> windows_core::Result<()> {
    windows_link::link!("drt.dll" "system" fn DrtUpdateKey(hkeyregistration : *const core::ffi::c_void, pappdata : *const DRT_DATA) -> windows_core::HRESULT);
    unsafe { DrtUpdateKey(hkeyregistration, pappdata).ok() }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientAddContentInformation(hpeerdist : isize, hcontenthandle : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientAddContentInformation(hpeerdist, hcontenthandle, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientAddData(hpeerdist : isize, hcontenthandle : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientAddData(hpeerdist, hcontenthandle, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, pbuffer: Option<&mut [u8]>, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientBlockRead(hpeerdist : isize, hcontenthandle : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, dwtimeoutinmilliseconds : u32, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientBlockRead(hpeerdist, hcontenthandle, pbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwtimeoutinmilliseconds, lpoverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientCancelAsyncOperation(hpeerdist : isize, hcontenthandle : isize, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientCancelAsyncOperation(hpeerdist, hcontenthandle, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientCloseContent(hpeerdist : isize, hcontenthandle : isize) -> u32);
    unsafe { PeerDistClientCloseContent(hpeerdist, hcontenthandle) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientCompleteContentInformation(hpeerdist : isize, hcontenthandle : isize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientCompleteContentInformation(hpeerdist, hcontenthandle, lpoverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientFlushContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientFlushContent(hpeerdist : isize, pcontenttag : *const PEERDIST_CONTENT_TAG, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientFlushContent(hpeerdist, pcontenttag, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, lpoverlapped) }
}
#[inline]
pub unsafe fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut core::ffi::c_void) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientGetInformationByHandle(hpeerdist : isize, hcontenthandle : isize, peerdistclientinfoclass : PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize : u32, lpinformation : *mut core::ffi::c_void) -> u32);
    unsafe { PeerDistClientGetInformationByHandle(hpeerdist, hcontenthandle, peerdistclientinfoclass, dwbuffersize, lpinformation as _) }
}
#[inline]
pub unsafe fn PeerDistClientOpenContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, phcontenthandle: *mut isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientOpenContent(hpeerdist : isize, pcontenttag : *const PEERDIST_CONTENT_TAG, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontenthandle : *mut isize) -> u32);
    unsafe { PeerDistClientOpenContent(hpeerdist, pcontenttag, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, phcontenthandle as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, pbuffer: Option<&mut [u8]>, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistClientStreamRead(hpeerdist : isize, hcontenthandle : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, dwtimeoutinmilliseconds : u32, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistClientStreamRead(hpeerdist, hcontenthandle, pbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwtimeoutinmilliseconds, lpoverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistGetOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: bool) -> windows_core::BOOL {
    windows_link::link!("peerdist.dll" "system" fn PeerDistGetOverlappedResult(lpoverlapped : *const super::super::System::IO:: OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PeerDistGetOverlappedResult(lpoverlapped, lpnumberofbytestransferred as _, bwait.into()) }
}
#[inline]
pub unsafe fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistGetStatus(hpeerdist : isize, ppeerdiststatus : *mut PEERDIST_STATUS) -> u32);
    unsafe { PeerDistGetStatus(hpeerdist, ppeerdiststatus as _) }
}
#[inline]
pub unsafe fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistGetStatusEx(hpeerdist : isize, ppeerdiststatus : *mut PEERDIST_STATUS_INFO) -> u32);
    unsafe { PeerDistGetStatusEx(hpeerdist, ppeerdiststatus as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotification(hpeerdist: isize, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistRegisterForStatusChangeNotification(hpeerdist : isize, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED, ppeerdiststatus : *mut PEERDIST_STATUS) -> u32);
    unsafe { PeerDistRegisterForStatusChangeNotification(hpeerdist, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, lpoverlapped, ppeerdiststatus as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist: isize, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist : isize, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED, ppeerdiststatus : *mut PEERDIST_STATUS_INFO) -> u32);
    unsafe { PeerDistRegisterForStatusChangeNotificationEx(hpeerdist, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, lpoverlapped, ppeerdiststatus as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, pcontentidentifier: &[u8], poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerCancelAsyncOperation(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistServerCancelAsyncOperation(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), poverlapped) }
}
#[inline]
pub unsafe fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerCloseContentInformation(hpeerdist : isize, hcontentinfo : isize) -> u32);
    unsafe { PeerDistServerCloseContentInformation(hpeerdist, hcontentinfo) }
}
#[inline]
pub unsafe fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerCloseStreamHandle(hpeerdist : isize, hstream : isize) -> u32);
    unsafe { PeerDistServerCloseStreamHandle(hpeerdist, hstream) }
}
#[inline]
pub unsafe fn PeerDistServerOpenContentInformation(hpeerdist: isize, pcontentidentifier: &[u8], ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, phcontentinfo: *mut isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerOpenContentInformation(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, ullcontentoffset : u64, cbcontentlength : u64, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontentinfo : *mut isize) -> u32);
    unsafe { PeerDistServerOpenContentInformation(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), ullcontentoffset, cbcontentlength, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, phcontentinfo as _) }
}
#[inline]
pub unsafe fn PeerDistServerOpenContentInformationEx(hpeerdist: isize, pcontentidentifier: &[u8], ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, phcontentinfo: *mut isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerOpenContentInformationEx(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, ullcontentoffset : u64, cbcontentlength : u64, pretrievaloptions : *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontentinfo : *mut isize) -> u32);
    unsafe { PeerDistServerOpenContentInformationEx(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), ullcontentoffset, cbcontentlength, pretrievaloptions, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, phcontentinfo as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerPublishAddToStream(hpeerdist : isize, hstream : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistServerPublishAddToStream(hpeerdist, hstream, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerPublishCompleteStream(hpeerdist : isize, hstream : isize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistServerPublishCompleteStream(hpeerdist, hstream, lpoverlapped) }
}
#[inline]
pub unsafe fn PeerDistServerPublishStream(hpeerdist: isize, pcontentidentifier: &[u8], cbcontentlength: u64, ppublishoptions: Option<*const PEERDIST_PUBLICATION_OPTIONS>, hcompletionport: Option<super::super::Foundation::HANDLE>, ulcompletionkey: Option<usize>, phstream: *mut isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerPublishStream(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, cbcontentlength : u64, ppublishoptions : *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phstream : *mut isize) -> u32);
    unsafe { PeerDistServerPublishStream(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), cbcontentlength, ppublishoptions.unwrap_or(core::mem::zeroed()) as _, hcompletionport.unwrap_or(core::mem::zeroed()) as _, ulcompletionkey.unwrap_or(core::mem::zeroed()) as _, phstream as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, pbuffer: &mut [u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerRetrieveContentInformation(hpeerdist : isize, hcontentinfo : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { PeerDistServerRetrieveContentInformation(hpeerdist, hcontentinfo, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped) }
}
#[inline]
pub unsafe fn PeerDistServerUnpublish(hpeerdist: isize, pcontentidentifier: &[u8]) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistServerUnpublish(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8) -> u32);
    unsafe { PeerDistServerUnpublish(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr())) }
}
#[inline]
pub unsafe fn PeerDistShutdown(hpeerdist: isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistShutdown(hpeerdist : isize) -> u32);
    unsafe { PeerDistShutdown(hpeerdist) }
}
#[inline]
pub unsafe fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: Option<*mut u32>) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistStartup(dwversionrequested : u32, phpeerdist : *mut isize, pdwsupportedversion : *mut u32) -> u32);
    unsafe { PeerDistStartup(dwversionrequested, phpeerdist as _, pdwsupportedversion.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32 {
    windows_link::link!("peerdist.dll" "system" fn PeerDistUnregisterForStatusChangeNotification(hpeerdist : isize) -> u32);
    unsafe { PeerDistUnregisterForStatusChangeNotification(hpeerdist) }
}
pub const DRT_ACTIVE: DRT_STATUS = DRT_STATUS(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_ADDRESS {
    pub socketAddress: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub flags: u32,
    pub nearness: i32,
    pub latency: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_ADDRESS_FLAGS(pub i32);
pub const DRT_ADDRESS_FLAG_ACCEPTED: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(1i32);
pub const DRT_ADDRESS_FLAG_BAD_VALIDATE_ID: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(32i32);
pub const DRT_ADDRESS_FLAG_INQUIRE: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(128i32);
pub const DRT_ADDRESS_FLAG_LOOP: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(8i32);
pub const DRT_ADDRESS_FLAG_REJECTED: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(2i32);
pub const DRT_ADDRESS_FLAG_SUSPECT_UNREGISTERED_ID: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(64i32);
pub const DRT_ADDRESS_FLAG_TOO_BUSY: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(16i32);
pub const DRT_ADDRESS_FLAG_UNREACHABLE: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(4i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_ADDRESS_LIST {
    pub AddressCount: u32,
    pub AddressList: [DRT_ADDRESS; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRT_ALONE: DRT_STATUS = DRT_STATUS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_BOOTSTRAP_PROVIDER {
    pub pvContext: *mut core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub InitResolve: isize,
    pub IssueResolve: isize,
    pub EndResolve: isize,
    pub Register: isize,
    pub Unregister: isize,
}
impl Default for DRT_BOOTSTRAP_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
pub type DRT_BOOTSTRAP_RESOLVE_CALLBACK = Option<unsafe extern "system" fn(hr: windows_core::HRESULT, pvcontext: *mut core::ffi::c_void, paddresses: *mut super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, ffatalerror: windows_core::BOOL)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_DATA {
    pub cb: u32,
    pub pb: *mut u8,
}
impl Default for DRT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA {
    pub r#type: DRT_EVENT_TYPE,
    pub hr: windows_core::HRESULT,
    pub pvContext: *mut core::ffi::c_void,
    pub Anonymous: DRT_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union DRT_EVENT_DATA_0 {
    pub leafsetKeyChange: DRT_EVENT_DATA_0_0,
    pub registrationStateChange: DRT_EVENT_DATA_0_1,
    pub statusChange: DRT_EVENT_DATA_0_2,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_EVENT_DATA_0_0 {
    pub change: DRT_LEAFSET_KEY_CHANGE_TYPE,
    pub localKey: DRT_DATA,
    pub remoteKey: DRT_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_EVENT_DATA_0_1 {
    pub state: DRT_REGISTRATION_STATE,
    pub localKey: DRT_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_EVENT_DATA_0_2 {
    pub status: DRT_STATUS,
    pub bootstrapAddresses: DRT_EVENT_DATA_0_2_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_EVENT_DATA_0_2_0 {
    pub cntAddress: u32,
    pub pAddresses: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRT_EVENT_LEAFSET_KEY_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(1i32);
pub const DRT_EVENT_REGISTRATION_STATE_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(2i32);
pub const DRT_EVENT_STATUS_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_EVENT_TYPE(pub i32);
pub const DRT_E_BOOTSTRAPPROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8062200E_u32 as _);
pub const DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8062200F_u32 as _);
pub const DRT_E_CAPABILITY_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8062210F_u32 as _);
pub const DRT_E_DUPLICATE_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80622009_u32 as _);
pub const DRT_E_FAULTED: windows_core::HRESULT = windows_core::HRESULT(0x8062210A_u32 as _);
pub const DRT_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x8062210C_u32 as _);
pub const DRT_E_INVALID_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80622005_u32 as _);
pub const DRT_E_INVALID_BOOTSTRAP_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80622004_u32 as _);
pub const DRT_E_INVALID_CERT_CHAIN: windows_core::HRESULT = windows_core::HRESULT(0x80621004_u32 as _);
pub const DRT_E_INVALID_INSTANCE_PREFIX: windows_core::HRESULT = windows_core::HRESULT(0x8062210D_u32 as _);
pub const DRT_E_INVALID_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80621009_u32 as _);
pub const DRT_E_INVALID_KEY_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80621002_u32 as _);
pub const DRT_E_INVALID_MAX_ADDRESSES: windows_core::HRESULT = windows_core::HRESULT(0x80621007_u32 as _);
pub const DRT_E_INVALID_MAX_ENDPOINTS: windows_core::HRESULT = windows_core::HRESULT(0x80621011_u32 as _);
pub const DRT_E_INVALID_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80621005_u32 as _);
pub const DRT_E_INVALID_PORT: windows_core::HRESULT = windows_core::HRESULT(0x80622000_u32 as _);
pub const DRT_E_INVALID_SCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80622006_u32 as _);
pub const DRT_E_INVALID_SEARCH_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80622109_u32 as _);
pub const DRT_E_INVALID_SEARCH_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80621012_u32 as _);
pub const DRT_E_INVALID_SECURITY_MODE: windows_core::HRESULT = windows_core::HRESULT(0x8062210E_u32 as _);
pub const DRT_E_INVALID_SECURITY_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80622002_u32 as _);
pub const DRT_E_INVALID_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80622108_u32 as _);
pub const DRT_E_INVALID_TRANSPORT_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80622001_u32 as _);
pub const DRT_E_NO_ADDRESSES_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80622008_u32 as _);
pub const DRT_E_NO_MORE: windows_core::HRESULT = windows_core::HRESULT(0x80621006_u32 as _);
pub const DRT_E_SEARCH_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80621008_u32 as _);
pub const DRT_E_SECURITYPROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8062200C_u32 as _);
pub const DRT_E_SECURITYPROVIDER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8062200D_u32 as _);
pub const DRT_E_STILL_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80622003_u32 as _);
pub const DRT_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80621001_u32 as _);
pub const DRT_E_TRANSPORTPROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8062200A_u32 as _);
pub const DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8062200B_u32 as _);
pub const DRT_E_TRANSPORT_ALREADY_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80622101_u32 as _);
pub const DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80622107_u32 as _);
pub const DRT_E_TRANSPORT_EXECUTING_CALLBACK: windows_core::HRESULT = windows_core::HRESULT(0x80622106_u32 as _);
pub const DRT_E_TRANSPORT_INVALID_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80622104_u32 as _);
pub const DRT_E_TRANSPORT_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80622102_u32 as _);
pub const DRT_E_TRANSPORT_NO_DEST_ADDRESSES: windows_core::HRESULT = windows_core::HRESULT(0x80622105_u32 as _);
pub const DRT_E_TRANSPORT_SHUTTING_DOWN: windows_core::HRESULT = windows_core::HRESULT(0x80622007_u32 as _);
pub const DRT_E_TRANSPORT_STILL_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x8062210B_u32 as _);
pub const DRT_E_TRANSPORT_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80622103_u32 as _);
pub const DRT_FAULTED: DRT_STATUS = DRT_STATUS(20i32);
pub const DRT_GLOBAL_SCOPE: DRT_SCOPE = DRT_SCOPE(1i32);
pub const DRT_LEAFSET_KEY_ADDED: DRT_LEAFSET_KEY_CHANGE_TYPE = DRT_LEAFSET_KEY_CHANGE_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_LEAFSET_KEY_CHANGE_TYPE(pub i32);
pub const DRT_LEAFSET_KEY_DELETED: DRT_LEAFSET_KEY_CHANGE_TYPE = DRT_LEAFSET_KEY_CHANGE_TYPE(1i32);
pub const DRT_LINK_LOCAL_ISATAP_SCOPEID: u32 = 4294967295u32;
pub const DRT_LINK_LOCAL_SCOPE: DRT_SCOPE = DRT_SCOPE(3i32);
pub const DRT_MATCH_EXACT: DRT_MATCH_TYPE = DRT_MATCH_TYPE(0i32);
pub const DRT_MATCH_INTERMEDIATE: DRT_MATCH_TYPE = DRT_MATCH_TYPE(2i32);
pub const DRT_MATCH_NEAR: DRT_MATCH_TYPE = DRT_MATCH_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_MATCH_TYPE(pub i32);
pub const DRT_MAX_INSTANCE_PREFIX_LEN: u32 = 128u32;
pub const DRT_MAX_PAYLOAD_SIZE: u32 = 5120u32;
pub const DRT_MAX_ROUTING_ADDRESSES: u32 = 20u32;
pub const DRT_MIN_ROUTING_ADDRESSES: u32 = 1u32;
pub const DRT_NO_NETWORK: DRT_STATUS = DRT_STATUS(10i32);
pub const DRT_PAYLOAD_REVOKED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_REGISTRATION {
    pub key: DRT_DATA,
    pub appData: DRT_DATA,
}
impl Default for DRT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_REGISTRATION_STATE(pub i32);
pub const DRT_REGISTRATION_STATE_UNRESOLVEABLE: DRT_REGISTRATION_STATE = DRT_REGISTRATION_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_SCOPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_SEARCH_INFO {
    pub dwSize: u32,
    pub fIterative: windows_core::BOOL,
    pub fAllowCurrentInstanceMatch: windows_core::BOOL,
    pub fAnyMatchInRange: windows_core::BOOL,
    pub cMaxEndpoints: u32,
    pub pMaximumKey: *mut DRT_DATA,
    pub pMinimumKey: *mut DRT_DATA,
}
impl Default for DRT_SEARCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_SEARCH_RESULT {
    pub dwSize: u32,
    pub r#type: DRT_MATCH_TYPE,
    pub pvContext: *mut core::ffi::c_void,
    pub registration: DRT_REGISTRATION,
}
impl Default for DRT_SEARCH_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRT_SECURE_CONFIDENTIALPAYLOAD: DRT_SECURITY_MODE = DRT_SECURITY_MODE(2i32);
pub const DRT_SECURE_MEMBERSHIP: DRT_SECURITY_MODE = DRT_SECURITY_MODE(1i32);
pub const DRT_SECURE_RESOLVE: DRT_SECURITY_MODE = DRT_SECURITY_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_SECURITY_MODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_SECURITY_PROVIDER {
    pub pvContext: *mut core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub RegisterKey: isize,
    pub UnregisterKey: isize,
    pub ValidateAndUnpackPayload: isize,
    pub SecureAndPackPayload: isize,
    pub FreeData: isize,
    pub EncryptData: isize,
    pub DecryptData: isize,
    pub GetSerializedCredential: isize,
    pub ValidateRemoteCredential: isize,
    pub SignData: isize,
    pub VerifyData: isize,
}
impl Default for DRT_SECURITY_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRT_SETTINGS {
    pub dwSize: u32,
    pub cbKey: u32,
    pub bProtocolMajorVersion: u8,
    pub bProtocolMinorVersion: u8,
    pub ulMaxRoutingAddresses: u32,
    pub pwzDrtInstancePrefix: windows_core::PWSTR,
    pub hTransport: *mut core::ffi::c_void,
    pub pSecurityProvider: *mut DRT_SECURITY_PROVIDER,
    pub pBootstrapProvider: *mut DRT_BOOTSTRAP_PROVIDER,
    pub eSecurityMode: DRT_SECURITY_MODE,
}
impl Default for DRT_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRT_SITE_LOCAL_SCOPE: DRT_SCOPE = DRT_SCOPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRT_STATUS(pub i32);
pub const DRT_S_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x621010_u32 as _);
pub const FACILITY_DRT: u32 = 98u32;
pub const MaximumPeerDistClientInfoByHandlesClass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(1i32);
pub const NS_PNRPCLOUD: u32 = 39u32;
pub const NS_PNRPNAME: u32 = 38u32;
pub const NS_PROVIDER_PNRPCLOUD: windows_core::GUID = windows_core::GUID::from_u128(0x03fe89ce_766d_4976_b9c1_bb9bc42c7b4d);
pub const NS_PROVIDER_PNRPNAME: windows_core::GUID = windows_core::GUID::from_u128(0x03fe89cd_766d_4976_b9c1_bb9bc42c7b4d);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEERDIST_CLIENT_BASIC_INFO {
    pub fFlashCrowd: windows_core::BOOL,
}
impl Default for PEERDIST_CLIENT_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEERDIST_CONTENT_TAG {
    pub Data: [u8; 16],
}
impl Default for PEERDIST_CONTENT_TAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEERDIST_PUBLICATION_OPTIONS {
    pub dwVersion: u32,
    pub dwFlags: u32,
}
impl Default for PEERDIST_PUBLICATION_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION: i32 = 2i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_1: i32 = 1i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_2: i32 = 2i32;
pub const PEERDIST_READ_TIMEOUT_DEFAULT: u32 = 4294967294u32;
pub const PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEERDIST_RETRIEVAL_OPTIONS {
    pub cbSize: u32,
    pub dwContentInfoMinVersion: u32,
    pub dwContentInfoMaxVersion: u32,
    pub dwReserved: u32,
}
impl Default for PEERDIST_RETRIEVAL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(2u32);
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(1u32);
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PEERDIST_STATUS(pub i32);
pub const PEERDIST_STATUS_AVAILABLE: PEERDIST_STATUS = PEERDIST_STATUS(2i32);
pub const PEERDIST_STATUS_DISABLED: PEERDIST_STATUS = PEERDIST_STATUS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEERDIST_STATUS_INFO {
    pub cbSize: u32,
    pub status: PEERDIST_STATUS,
    pub dwMinVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
    pub dwMaxVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
}
impl Default for PEERDIST_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PEERDIST_STATUS_UNAVAILABLE: PEERDIST_STATUS = PEERDIST_STATUS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNRPCLOUDINFO {
    pub dwSize: u32,
    pub Cloud: PNRP_CLOUD_ID,
    pub enCloudState: PNRP_CLOUD_STATE,
    pub enCloudFlags: PNRP_CLOUD_FLAGS,
}
impl Default for PNRPCLOUDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PNRPINFO_HINT: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNRPINFO_V1 {
    pub dwSize: u32,
    pub lpwszIdentity: windows_core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PNRPINFO_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub struct PNRPINFO_V2 {
    pub dwSize: u32,
    pub lpwszIdentity: windows_core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
    pub enExtendedPayloadType: PNRP_EXTENDED_PAYLOAD_TYPE,
    pub Anonymous: PNRPINFO_V2_0,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl Default for PNRPINFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub union PNRPINFO_V2_0 {
    pub blobPayload: super::super::System::Com::BLOB,
    pub pwszPayload: windows_core::PWSTR,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl Default for PNRPINFO_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNRP_CLOUD_FLAGS(pub i32);
pub const PNRP_CLOUD_FULL_PARTICIPANT: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNRP_CLOUD_ID {
    pub AddressFamily: i32,
    pub Scope: PNRP_SCOPE,
    pub ScopeId: u32,
}
impl Default for PNRP_CLOUD_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PNRP_CLOUD_NAME_LOCAL: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(1i32);
pub const PNRP_CLOUD_NO_FLAGS: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(0i32);
pub const PNRP_CLOUD_RESOLVE_ONLY: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNRP_CLOUD_STATE(pub i32);
pub const PNRP_CLOUD_STATE_ACTIVE: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(2i32);
pub const PNRP_CLOUD_STATE_ALONE: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(6i32);
pub const PNRP_CLOUD_STATE_DEAD: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(3i32);
pub const PNRP_CLOUD_STATE_DISABLED: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(4i32);
pub const PNRP_CLOUD_STATE_NO_NET: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(5i32);
pub const PNRP_CLOUD_STATE_SYNCHRONISING: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(1i32);
pub const PNRP_CLOUD_STATE_VIRTUAL: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNRP_EXTENDED_PAYLOAD_TYPE(pub i32);
pub const PNRP_EXTENDED_PAYLOAD_TYPE_BINARY: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(1i32);
pub const PNRP_EXTENDED_PAYLOAD_TYPE_NONE: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(0i32);
pub const PNRP_EXTENDED_PAYLOAD_TYPE_STRING: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(2i32);
pub const PNRP_GLOBAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(1i32);
pub const PNRP_LINK_LOCAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(3i32);
pub const PNRP_MAX_ENDPOINT_ADDRESSES: u32 = 10u32;
pub const PNRP_MAX_EXTENDED_PAYLOAD_BYTES: u32 = 4096u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNRP_REGISTERED_ID_STATE(pub i32);
pub const PNRP_REGISTERED_ID_STATE_OK: PNRP_REGISTERED_ID_STATE = PNRP_REGISTERED_ID_STATE(1i32);
pub const PNRP_REGISTERED_ID_STATE_PROBLEM: PNRP_REGISTERED_ID_STATE = PNRP_REGISTERED_ID_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNRP_RESOLVE_CRITERIA(pub i32);
pub const PNRP_RESOLVE_CRITERIA_ANY_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(5i32);
pub const PNRP_RESOLVE_CRITERIA_DEFAULT: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(0i32);
pub const PNRP_RESOLVE_CRITERIA_NEAREST_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(4i32);
pub const PNRP_RESOLVE_CRITERIA_NEAREST_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(6i32);
pub const PNRP_RESOLVE_CRITERIA_NEAREST_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(2i32);
pub const PNRP_RESOLVE_CRITERIA_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(3i32);
pub const PNRP_RESOLVE_CRITERIA_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNRP_SCOPE(pub i32);
pub const PNRP_SCOPE_ANY: PNRP_SCOPE = PNRP_SCOPE(0i32);
pub const PNRP_SITE_LOCAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(2i32);
pub const PeerDistClientBasicInfo: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(0i32);
pub const SVCID_PNRPCLOUD: windows_core::GUID = windows_core::GUID::from_u128(0xc2239ce6_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V1: windows_core::GUID = windows_core::GUID::from_u128(0xc2239ce5_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V2: windows_core::GUID = windows_core::GUID::from_u128(0xc2239ce7_00c0_4fbf_bad6_18139385a49a);
pub const WSZ_SCOPE_GLOBAL: windows_core::PCWSTR = windows_core::w!("GLOBAL");
pub const WSZ_SCOPE_LINKLOCAL: windows_core::PCWSTR = windows_core::w!("LINKLOCAL");
pub const WSZ_SCOPE_SITELOCAL: windows_core::PCWSTR = windows_core::w!("SITELOCAL");
