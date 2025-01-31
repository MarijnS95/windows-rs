windows_targets::link!("drt.dll" "system" fn DrtClose(hdrt : *const core::ffi::c_void));
windows_targets::link!("drt.dll" "system" fn DrtContinueSearch(hsearchcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security_Cryptography")]
windows_targets::link!("drtprov.dll" "system" fn DrtCreateDerivedKey(plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, pkey : *mut DRT_DATA) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security_Cryptography")]
windows_targets::link!("drtprov.dll" "system" fn DrtCreateDerivedKeySecurityProvider(prootcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER) -> windows_sys::core::HRESULT);
windows_targets::link!("drtprov.dll" "system" fn DrtCreateDnsBootstrapResolver(port : u16, pwszaddress : windows_sys::core::PCWSTR, ppmodule : *mut *mut DRT_BOOTSTRAP_PROVIDER) -> windows_sys::core::HRESULT);
windows_targets::link!("drttransport.dll" "system" fn DrtCreateIpv6UdpTransport(scope : DRT_SCOPE, dwscopeid : u32, dwlocalitythreshold : u32, pwport : *mut u16, phtransport : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("drtprov.dll" "system" fn DrtCreateNullSecurityProvider(ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER) -> windows_sys::core::HRESULT);
windows_targets::link!("drtprov.dll" "system" fn DrtCreatePnrpBootstrapResolver(fpublish : windows_sys::core::BOOL, pwzpeername : windows_sys::core::PCWSTR, pwzcloudname : windows_sys::core::PCWSTR, pwzpublishingidentity : windows_sys::core::PCWSTR, ppresolver : *mut *mut DRT_BOOTSTRAP_PROVIDER) -> windows_sys::core::HRESULT);
windows_targets::link!("drtprov.dll" "system" fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider : *const DRT_SECURITY_PROVIDER));
windows_targets::link!("drtprov.dll" "system" fn DrtDeleteDnsBootstrapResolver(presolver : *const DRT_BOOTSTRAP_PROVIDER));
windows_targets::link!("drttransport.dll" "system" fn DrtDeleteIpv6UdpTransport(htransport : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("drtprov.dll" "system" fn DrtDeleteNullSecurityProvider(psecurityprovider : *const DRT_SECURITY_PROVIDER));
windows_targets::link!("drtprov.dll" "system" fn DrtDeletePnrpBootstrapResolver(presolver : *const DRT_BOOTSTRAP_PROVIDER));
windows_targets::link!("drt.dll" "system" fn DrtEndSearch(hsearchcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Networking_WinSock")]
windows_targets::link!("drt.dll" "system" fn DrtGetEventData(hdrt : *const core::ffi::c_void, uleventdatalen : u32, peventdata : *mut DRT_EVENT_DATA) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtGetEventDataSize(hdrt : *const core::ffi::c_void, puleventdatalen : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtGetInstanceName(hdrt : *const core::ffi::c_void, ulcbinstancenamesize : u32, pwzdrtinstancename : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtGetInstanceNameSize(hdrt : *const core::ffi::c_void, pulcbinstancenamesize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Networking_WinSock")]
windows_targets::link!("drt.dll" "system" fn DrtGetSearchPath(hsearchcontext : *const core::ffi::c_void, ulsearchpathsize : u32, psearchpath : *mut DRT_ADDRESS_LIST) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtGetSearchPathSize(hsearchcontext : *const core::ffi::c_void, pulsearchpathsize : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtGetSearchResult(hsearchcontext : *const core::ffi::c_void, ulsearchresultsize : u32, psearchresult : *mut DRT_SEARCH_RESULT) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtGetSearchResultSize(hsearchcontext : *const core::ffi::c_void, pulsearchresultsize : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtOpen(psettings : *const DRT_SETTINGS, hevent : super::super::Foundation:: HANDLE, pvcontext : *const core::ffi::c_void, phdrt : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtRegisterKey(hdrt : *const core::ffi::c_void, pregistration : *const DRT_REGISTRATION, pvkeycontext : *const core::ffi::c_void, phkeyregistration : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtStartSearch(hdrt : *const core::ffi::c_void, pkey : *const DRT_DATA, pinfo : *const DRT_SEARCH_INFO, timeout : u32, hevent : super::super::Foundation:: HANDLE, pvcontext : *const core::ffi::c_void, hsearchcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("drt.dll" "system" fn DrtUnregisterKey(hkeyregistration : *const core::ffi::c_void));
windows_targets::link!("drt.dll" "system" fn DrtUpdateKey(hkeyregistration : *const core::ffi::c_void, pappdata : *const DRT_DATA) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientAddContentInformation(hpeerdist : isize, hcontenthandle : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientAddData(hpeerdist : isize, hcontenthandle : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientBlockRead(hpeerdist : isize, hcontenthandle : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, dwtimeoutinmilliseconds : u32, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientCancelAsyncOperation(hpeerdist : isize, hcontenthandle : isize, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientCloseContent(hpeerdist : isize, hcontenthandle : isize) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientCompleteContentInformation(hpeerdist : isize, hcontenthandle : isize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientFlushContent(hpeerdist : isize, pcontenttag : *const PEERDIST_CONTENT_TAG, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientGetInformationByHandle(hpeerdist : isize, hcontenthandle : isize, peerdistclientinfoclass : PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize : u32, lpinformation : *mut core::ffi::c_void) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientOpenContent(hpeerdist : isize, pcontenttag : *const PEERDIST_CONTENT_TAG, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontenthandle : *mut isize) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistClientStreamRead(hpeerdist : isize, hcontenthandle : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, dwtimeoutinmilliseconds : u32, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistGetOverlappedResult(lpoverlapped : *const super::super::System::IO:: OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_targets::link!("peerdist.dll" "system" fn PeerDistGetStatus(hpeerdist : isize, ppeerdiststatus : *mut PEERDIST_STATUS) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistGetStatusEx(hpeerdist : isize, ppeerdiststatus : *mut PEERDIST_STATUS_INFO) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistRegisterForStatusChangeNotification(hpeerdist : isize, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED, ppeerdiststatus : *mut PEERDIST_STATUS) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist : isize, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED, ppeerdiststatus : *mut PEERDIST_STATUS_INFO) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerCancelAsyncOperation(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerCloseContentInformation(hpeerdist : isize, hcontentinfo : isize) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerCloseStreamHandle(hpeerdist : isize, hstream : isize) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerOpenContentInformation(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, ullcontentoffset : u64, cbcontentlength : u64, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontentinfo : *mut isize) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerOpenContentInformationEx(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, ullcontentoffset : u64, cbcontentlength : u64, pretrievaloptions : *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontentinfo : *mut isize) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerPublishAddToStream(hpeerdist : isize, hstream : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerPublishCompleteStream(hpeerdist : isize, hstream : isize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerPublishStream(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, cbcontentlength : u64, ppublishoptions : *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phstream : *mut isize) -> u32);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerRetrieveContentInformation(hpeerdist : isize, hcontentinfo : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistServerUnpublish(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistShutdown(hpeerdist : isize) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistStartup(dwversionrequested : u32, phpeerdist : *mut isize, pdwsupportedversion : *mut u32) -> u32);
windows_targets::link!("peerdist.dll" "system" fn PeerDistUnregisterForStatusChangeNotification(hpeerdist : isize) -> u32);
pub const DRT_ACTIVE: DRT_STATUS = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_ADDRESS {
    pub socketAddress: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub flags: u32,
    pub nearness: i32,
    pub latency: u32,
}
pub type DRT_ADDRESS_FLAGS = i32;
pub const DRT_ADDRESS_FLAG_ACCEPTED: DRT_ADDRESS_FLAGS = 1i32;
pub const DRT_ADDRESS_FLAG_BAD_VALIDATE_ID: DRT_ADDRESS_FLAGS = 32i32;
pub const DRT_ADDRESS_FLAG_INQUIRE: DRT_ADDRESS_FLAGS = 128i32;
pub const DRT_ADDRESS_FLAG_LOOP: DRT_ADDRESS_FLAGS = 8i32;
pub const DRT_ADDRESS_FLAG_REJECTED: DRT_ADDRESS_FLAGS = 2i32;
pub const DRT_ADDRESS_FLAG_SUSPECT_UNREGISTERED_ID: DRT_ADDRESS_FLAGS = 64i32;
pub const DRT_ADDRESS_FLAG_TOO_BUSY: DRT_ADDRESS_FLAGS = 16i32;
pub const DRT_ADDRESS_FLAG_UNREACHABLE: DRT_ADDRESS_FLAGS = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_ADDRESS_LIST {
    pub AddressCount: u32,
    pub AddressList: [DRT_ADDRESS; 1],
}
pub const DRT_ALONE: DRT_STATUS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[cfg(feature = "Win32_Networking_WinSock")]
pub type DRT_BOOTSTRAP_RESOLVE_CALLBACK = Option<unsafe extern "system" fn(hr: windows_sys::core::HRESULT, pvcontext: *mut core::ffi::c_void, paddresses: *mut super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, ffatalerror: windows_sys::core::BOOL)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRT_DATA {
    pub cb: u32,
    pub pb: *mut u8,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA {
    pub r#type: DRT_EVENT_TYPE,
    pub hr: windows_sys::core::HRESULT,
    pub pvContext: *mut core::ffi::c_void,
    pub Anonymous: DRT_EVENT_DATA_0,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union DRT_EVENT_DATA_0 {
    pub leafsetKeyChange: DRT_EVENT_DATA_0_0,
    pub registrationStateChange: DRT_EVENT_DATA_0_1,
    pub statusChange: DRT_EVENT_DATA_0_2,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA_0_0 {
    pub change: DRT_LEAFSET_KEY_CHANGE_TYPE,
    pub localKey: DRT_DATA,
    pub remoteKey: DRT_DATA,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA_0_1 {
    pub state: DRT_REGISTRATION_STATE,
    pub localKey: DRT_DATA,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA_0_2 {
    pub status: DRT_STATUS,
    pub bootstrapAddresses: DRT_EVENT_DATA_0_2_0,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA_0_2_0 {
    pub cntAddress: u32,
    pub pAddresses: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE,
}
pub const DRT_EVENT_LEAFSET_KEY_CHANGED: DRT_EVENT_TYPE = 1i32;
pub const DRT_EVENT_REGISTRATION_STATE_CHANGED: DRT_EVENT_TYPE = 2i32;
pub const DRT_EVENT_STATUS_CHANGED: DRT_EVENT_TYPE = 0i32;
pub type DRT_EVENT_TYPE = i32;
pub const DRT_E_BOOTSTRAPPROVIDER_IN_USE: windows_sys::core::HRESULT = 0x8062200E_u32 as _;
pub const DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED: windows_sys::core::HRESULT = 0x8062200F_u32 as _;
pub const DRT_E_CAPABILITY_MISMATCH: windows_sys::core::HRESULT = 0x8062210F_u32 as _;
pub const DRT_E_DUPLICATE_KEY: windows_sys::core::HRESULT = 0x80622009_u32 as _;
pub const DRT_E_FAULTED: windows_sys::core::HRESULT = 0x8062210A_u32 as _;
pub const DRT_E_INSUFFICIENT_BUFFER: windows_sys::core::HRESULT = 0x8062210C_u32 as _;
pub const DRT_E_INVALID_ADDRESS: windows_sys::core::HRESULT = 0x80622005_u32 as _;
pub const DRT_E_INVALID_BOOTSTRAP_PROVIDER: windows_sys::core::HRESULT = 0x80622004_u32 as _;
pub const DRT_E_INVALID_CERT_CHAIN: windows_sys::core::HRESULT = 0x80621004_u32 as _;
pub const DRT_E_INVALID_INSTANCE_PREFIX: windows_sys::core::HRESULT = 0x8062210D_u32 as _;
pub const DRT_E_INVALID_KEY: windows_sys::core::HRESULT = 0x80621009_u32 as _;
pub const DRT_E_INVALID_KEY_SIZE: windows_sys::core::HRESULT = 0x80621002_u32 as _;
pub const DRT_E_INVALID_MAX_ADDRESSES: windows_sys::core::HRESULT = 0x80621007_u32 as _;
pub const DRT_E_INVALID_MAX_ENDPOINTS: windows_sys::core::HRESULT = 0x80621011_u32 as _;
pub const DRT_E_INVALID_MESSAGE: windows_sys::core::HRESULT = 0x80621005_u32 as _;
pub const DRT_E_INVALID_PORT: windows_sys::core::HRESULT = 0x80622000_u32 as _;
pub const DRT_E_INVALID_SCOPE: windows_sys::core::HRESULT = 0x80622006_u32 as _;
pub const DRT_E_INVALID_SEARCH_INFO: windows_sys::core::HRESULT = 0x80622109_u32 as _;
pub const DRT_E_INVALID_SEARCH_RANGE: windows_sys::core::HRESULT = 0x80621012_u32 as _;
pub const DRT_E_INVALID_SECURITY_MODE: windows_sys::core::HRESULT = 0x8062210E_u32 as _;
pub const DRT_E_INVALID_SECURITY_PROVIDER: windows_sys::core::HRESULT = 0x80622002_u32 as _;
pub const DRT_E_INVALID_SETTINGS: windows_sys::core::HRESULT = 0x80622108_u32 as _;
pub const DRT_E_INVALID_TRANSPORT_PROVIDER: windows_sys::core::HRESULT = 0x80622001_u32 as _;
pub const DRT_E_NO_ADDRESSES_AVAILABLE: windows_sys::core::HRESULT = 0x80622008_u32 as _;
pub const DRT_E_NO_MORE: windows_sys::core::HRESULT = 0x80621006_u32 as _;
pub const DRT_E_SEARCH_IN_PROGRESS: windows_sys::core::HRESULT = 0x80621008_u32 as _;
pub const DRT_E_SECURITYPROVIDER_IN_USE: windows_sys::core::HRESULT = 0x8062200C_u32 as _;
pub const DRT_E_SECURITYPROVIDER_NOT_ATTACHED: windows_sys::core::HRESULT = 0x8062200D_u32 as _;
pub const DRT_E_STILL_IN_USE: windows_sys::core::HRESULT = 0x80622003_u32 as _;
pub const DRT_E_TIMEOUT: windows_sys::core::HRESULT = 0x80621001_u32 as _;
pub const DRT_E_TRANSPORTPROVIDER_IN_USE: windows_sys::core::HRESULT = 0x8062200A_u32 as _;
pub const DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED: windows_sys::core::HRESULT = 0x8062200B_u32 as _;
pub const DRT_E_TRANSPORT_ALREADY_BOUND: windows_sys::core::HRESULT = 0x80622101_u32 as _;
pub const DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE: windows_sys::core::HRESULT = 0x80622107_u32 as _;
pub const DRT_E_TRANSPORT_EXECUTING_CALLBACK: windows_sys::core::HRESULT = 0x80622106_u32 as _;
pub const DRT_E_TRANSPORT_INVALID_ARGUMENT: windows_sys::core::HRESULT = 0x80622104_u32 as _;
pub const DRT_E_TRANSPORT_NOT_BOUND: windows_sys::core::HRESULT = 0x80622102_u32 as _;
pub const DRT_E_TRANSPORT_NO_DEST_ADDRESSES: windows_sys::core::HRESULT = 0x80622105_u32 as _;
pub const DRT_E_TRANSPORT_SHUTTING_DOWN: windows_sys::core::HRESULT = 0x80622007_u32 as _;
pub const DRT_E_TRANSPORT_STILL_BOUND: windows_sys::core::HRESULT = 0x8062210B_u32 as _;
pub const DRT_E_TRANSPORT_UNEXPECTED: windows_sys::core::HRESULT = 0x80622103_u32 as _;
pub const DRT_FAULTED: DRT_STATUS = 20i32;
pub const DRT_GLOBAL_SCOPE: DRT_SCOPE = 1i32;
pub const DRT_LEAFSET_KEY_ADDED: DRT_LEAFSET_KEY_CHANGE_TYPE = 0i32;
pub type DRT_LEAFSET_KEY_CHANGE_TYPE = i32;
pub const DRT_LEAFSET_KEY_DELETED: DRT_LEAFSET_KEY_CHANGE_TYPE = 1i32;
pub const DRT_LINK_LOCAL_ISATAP_SCOPEID: u32 = 4294967295u32;
pub const DRT_LINK_LOCAL_SCOPE: DRT_SCOPE = 3i32;
pub const DRT_MATCH_EXACT: DRT_MATCH_TYPE = 0i32;
pub const DRT_MATCH_INTERMEDIATE: DRT_MATCH_TYPE = 2i32;
pub const DRT_MATCH_NEAR: DRT_MATCH_TYPE = 1i32;
pub type DRT_MATCH_TYPE = i32;
pub const DRT_MAX_INSTANCE_PREFIX_LEN: u32 = 128u32;
pub const DRT_MAX_PAYLOAD_SIZE: u32 = 5120u32;
pub const DRT_MAX_ROUTING_ADDRESSES: u32 = 20u32;
pub const DRT_MIN_ROUTING_ADDRESSES: u32 = 1u32;
pub const DRT_NO_NETWORK: DRT_STATUS = 10i32;
pub const DRT_PAYLOAD_REVOKED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRT_REGISTRATION {
    pub key: DRT_DATA,
    pub appData: DRT_DATA,
}
pub type DRT_REGISTRATION_STATE = i32;
pub const DRT_REGISTRATION_STATE_UNRESOLVEABLE: DRT_REGISTRATION_STATE = 1i32;
pub type DRT_SCOPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRT_SEARCH_INFO {
    pub dwSize: u32,
    pub fIterative: windows_sys::core::BOOL,
    pub fAllowCurrentInstanceMatch: windows_sys::core::BOOL,
    pub fAnyMatchInRange: windows_sys::core::BOOL,
    pub cMaxEndpoints: u32,
    pub pMaximumKey: *mut DRT_DATA,
    pub pMinimumKey: *mut DRT_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRT_SEARCH_RESULT {
    pub dwSize: u32,
    pub r#type: DRT_MATCH_TYPE,
    pub pvContext: *mut core::ffi::c_void,
    pub registration: DRT_REGISTRATION,
}
pub const DRT_SECURE_CONFIDENTIALPAYLOAD: DRT_SECURITY_MODE = 2i32;
pub const DRT_SECURE_MEMBERSHIP: DRT_SECURITY_MODE = 1i32;
pub const DRT_SECURE_RESOLVE: DRT_SECURITY_MODE = 0i32;
pub type DRT_SECURITY_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRT_SETTINGS {
    pub dwSize: u32,
    pub cbKey: u32,
    pub bProtocolMajorVersion: u8,
    pub bProtocolMinorVersion: u8,
    pub ulMaxRoutingAddresses: u32,
    pub pwzDrtInstancePrefix: windows_sys::core::PWSTR,
    pub hTransport: *mut core::ffi::c_void,
    pub pSecurityProvider: *mut DRT_SECURITY_PROVIDER,
    pub pBootstrapProvider: *mut DRT_BOOTSTRAP_PROVIDER,
    pub eSecurityMode: DRT_SECURITY_MODE,
}
pub const DRT_SITE_LOCAL_SCOPE: DRT_SCOPE = 2i32;
pub type DRT_STATUS = i32;
pub const DRT_S_RETRY: windows_sys::core::HRESULT = 0x621010_u32 as _;
pub const FACILITY_DRT: u32 = 98u32;
pub const MaximumPeerDistClientInfoByHandlesClass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = 1i32;
pub const NS_PNRPCLOUD: u32 = 39u32;
pub const NS_PNRPNAME: u32 = 38u32;
pub const NS_PROVIDER_PNRPCLOUD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03fe89ce_766d_4976_b9c1_bb9bc42c7b4d);
pub const NS_PROVIDER_PNRPNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03fe89cd_766d_4976_b9c1_bb9bc42c7b4d);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEERDIST_CLIENT_BASIC_INFO {
    pub fFlashCrowd: windows_sys::core::BOOL,
}
pub type PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEERDIST_CONTENT_TAG {
    pub Data: [u8; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEERDIST_PUBLICATION_OPTIONS {
    pub dwVersion: u32,
    pub dwFlags: u32,
}
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION: i32 = 2i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_1: i32 = 1i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_2: i32 = 2i32;
pub const PEERDIST_READ_TIMEOUT_DEFAULT: u32 = 4294967294u32;
pub const PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEERDIST_RETRIEVAL_OPTIONS {
    pub cbSize: u32,
    pub dwContentInfoMinVersion: u32,
    pub dwContentInfoMaxVersion: u32,
    pub dwReserved: u32,
}
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 2u32;
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 1u32;
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 2u32;
pub type PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = u32;
pub type PEERDIST_STATUS = i32;
pub const PEERDIST_STATUS_AVAILABLE: PEERDIST_STATUS = 2i32;
pub const PEERDIST_STATUS_DISABLED: PEERDIST_STATUS = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEERDIST_STATUS_INFO {
    pub cbSize: u32,
    pub status: PEERDIST_STATUS,
    pub dwMinVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
    pub dwMaxVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
}
pub const PEERDIST_STATUS_UNAVAILABLE: PEERDIST_STATUS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNRPCLOUDINFO {
    pub dwSize: u32,
    pub Cloud: PNRP_CLOUD_ID,
    pub enCloudState: PNRP_CLOUD_STATE,
    pub enCloudFlags: PNRP_CLOUD_FLAGS,
}
pub const PNRPINFO_HINT: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct PNRPINFO_V1 {
    pub dwSize: u32,
    pub lpwszIdentity: windows_sys::core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub struct PNRPINFO_V2 {
    pub dwSize: u32,
    pub lpwszIdentity: windows_sys::core::PWSTR,
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
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub union PNRPINFO_V2_0 {
    pub blobPayload: super::super::System::Com::BLOB,
    pub pwszPayload: windows_sys::core::PWSTR,
}
pub type PNRP_CLOUD_FLAGS = i32;
pub const PNRP_CLOUD_FULL_PARTICIPANT: PNRP_CLOUD_FLAGS = 4i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNRP_CLOUD_ID {
    pub AddressFamily: i32,
    pub Scope: PNRP_SCOPE,
    pub ScopeId: u32,
}
pub const PNRP_CLOUD_NAME_LOCAL: PNRP_CLOUD_FLAGS = 1i32;
pub const PNRP_CLOUD_NO_FLAGS: PNRP_CLOUD_FLAGS = 0i32;
pub const PNRP_CLOUD_RESOLVE_ONLY: PNRP_CLOUD_FLAGS = 2i32;
pub type PNRP_CLOUD_STATE = i32;
pub const PNRP_CLOUD_STATE_ACTIVE: PNRP_CLOUD_STATE = 2i32;
pub const PNRP_CLOUD_STATE_ALONE: PNRP_CLOUD_STATE = 6i32;
pub const PNRP_CLOUD_STATE_DEAD: PNRP_CLOUD_STATE = 3i32;
pub const PNRP_CLOUD_STATE_DISABLED: PNRP_CLOUD_STATE = 4i32;
pub const PNRP_CLOUD_STATE_NO_NET: PNRP_CLOUD_STATE = 5i32;
pub const PNRP_CLOUD_STATE_SYNCHRONISING: PNRP_CLOUD_STATE = 1i32;
pub const PNRP_CLOUD_STATE_VIRTUAL: PNRP_CLOUD_STATE = 0i32;
pub type PNRP_EXTENDED_PAYLOAD_TYPE = i32;
pub const PNRP_EXTENDED_PAYLOAD_TYPE_BINARY: PNRP_EXTENDED_PAYLOAD_TYPE = 1i32;
pub const PNRP_EXTENDED_PAYLOAD_TYPE_NONE: PNRP_EXTENDED_PAYLOAD_TYPE = 0i32;
pub const PNRP_EXTENDED_PAYLOAD_TYPE_STRING: PNRP_EXTENDED_PAYLOAD_TYPE = 2i32;
pub const PNRP_GLOBAL_SCOPE: PNRP_SCOPE = 1i32;
pub const PNRP_LINK_LOCAL_SCOPE: PNRP_SCOPE = 3i32;
pub const PNRP_MAX_ENDPOINT_ADDRESSES: u32 = 10u32;
pub const PNRP_MAX_EXTENDED_PAYLOAD_BYTES: u32 = 4096u32;
pub type PNRP_REGISTERED_ID_STATE = i32;
pub const PNRP_REGISTERED_ID_STATE_OK: PNRP_REGISTERED_ID_STATE = 1i32;
pub const PNRP_REGISTERED_ID_STATE_PROBLEM: PNRP_REGISTERED_ID_STATE = 2i32;
pub type PNRP_RESOLVE_CRITERIA = i32;
pub const PNRP_RESOLVE_CRITERIA_ANY_PEER_NAME: PNRP_RESOLVE_CRITERIA = 5i32;
pub const PNRP_RESOLVE_CRITERIA_DEFAULT: PNRP_RESOLVE_CRITERIA = 0i32;
pub const PNRP_RESOLVE_CRITERIA_NEAREST_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = 4i32;
pub const PNRP_RESOLVE_CRITERIA_NEAREST_PEER_NAME: PNRP_RESOLVE_CRITERIA = 6i32;
pub const PNRP_RESOLVE_CRITERIA_NEAREST_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = 2i32;
pub const PNRP_RESOLVE_CRITERIA_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = 3i32;
pub const PNRP_RESOLVE_CRITERIA_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = 1i32;
pub type PNRP_SCOPE = i32;
pub const PNRP_SCOPE_ANY: PNRP_SCOPE = 0i32;
pub const PNRP_SITE_LOCAL_SCOPE: PNRP_SCOPE = 2i32;
pub const PeerDistClientBasicInfo: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = 0i32;
pub const SVCID_PNRPCLOUD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2239ce6_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2239ce5_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2239ce7_00c0_4fbf_bad6_18139385a49a);
pub const WSZ_SCOPE_GLOBAL: windows_sys::core::PCWSTR = windows_sys::core::w!("GLOBAL");
pub const WSZ_SCOPE_LINKLOCAL: windows_sys::core::PCWSTR = windows_sys::core::w!("LINKLOCAL");
pub const WSZ_SCOPE_SITELOCAL: windows_sys::core::PCWSTR = windows_sys::core::w!("SITELOCAL");
