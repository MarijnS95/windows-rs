#![allow(non_snake_case)]

use crate::Abi;
use widestring;

// demand_load! {
//     "OLEAUT32" {
//         fn SysStringLen(pbstr: BSTR_abi) -> u32;
//         // fn SysStringByteLen(pbstr: BSTR_abi) -> u32;
//         fn SysFreeString(pbstr: BSTR_abi) -> ();
//         fn SysAllocStringLen(strin: crate::PWSTR, ui: u32) -> BSTR;
//     }
// }

#[cfg(windows)]
#[link(name = "OLEAUT32")]
extern "system" {
    pub fn SysFreeString(bstrstring: BSTR_abi);
    pub fn SysAllocStringLen(strin: crate::PWSTR, ui: u32) -> BSTR;
    pub fn SysStringLen(pbstr: BSTR_abi) -> u32;
}

#[repr(transparent)]
#[derive(::std::cmp::Eq)]
/// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr#remarks
/// Uses [`widestring::UStr`] and not [`widestring::UCstr`], the latter checks for internal nulls.
pub struct BSTR(*mut widestring::WideChar);
impl BSTR {
    pub fn is_empty(&self) -> bool {
        // TODO: Should possibly also check length!
        self.0.is_null()
    }
    pub fn len(&self) -> usize {
        // TODO: Override directly in bindings.rs generation
        #[cfg(not(windows))]
        unsafe fn SysStringLen(s: BSTR_abi) -> u32 {
            unsafe fn SysStringByteLen(s: BSTR_abi) -> u32 {
                if s.is_null() {
                    0
                } else {
                    s.cast::<u32>().offset(-1).read()
                }
            }
            SysStringByteLen(s) / std::mem::size_of::<widestring::WideChar>() as u32
        }
        unsafe { SysStringLen(self.abi()) as usize }
    }
    fn from_wide(value: &widestring::WideStr) -> Self {
        if value.is_empty() {
            return Self(::std::ptr::null_mut());
        }
        #[cfg(windows)]
        unsafe {
            SysAllocStringLen(crate::PWSTR(value.as_ptr() as _), value.len() as u32)
        }

        // TODO: Implement this inside `SysAllocStringLen`
        #[cfg(not(windows))]
        {
            const LENGTH_PREFIX_IN_CHARS: usize =
                std::mem::size_of::<u32>() / std::mem::size_of::<widestring::WideChar>();
            let mut vec: Vec<widestring::WideChar> =
                        // Includes trailing null character
                        vec![0; LENGTH_PREFIX_IN_CHARS + value.len() + 1];
            vec[LENGTH_PREFIX_IN_CHARS..LENGTH_PREFIX_IN_CHARS + value.len()]
                .copy_from_slice(value.as_slice());
            assert_eq!(vec[LENGTH_PREFIX_IN_CHARS + value.len()], 0);

            // Cast to u32 for easy access to length prefix and
            // pointer beyond that
            let vec_ptr = vec.as_mut_ptr().cast::<u32>();
            std::mem::forget(vec);

            // Store length-prefix without NULL, in bytes
            unsafe {
                *vec_ptr = (value.len() * std::mem::size_of::<widestring::WideChar>()) as u32
            };

            BSTR(
                // Pointer points to first character byte, not to length-prefix
                unsafe { vec_ptr.offset(1) }.cast::<widestring::WideChar>(),
            )
        }
    }
    fn as_wide(&self) -> &widestring::WideStr {
        if self.0.is_null() {
            // `UStr` unlike `UCStr` doesn't implement an empty-string default yet
            widestring::WideStr::from_slice(&[])
        } else {
            unsafe { widestring::WideStr::from_ptr(self.0, self.len()) }
        }
    }
}
impl ::std::clone::Clone for BSTR {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_wide())
    }
}

impl ::std::convert::From<&str> for BSTR {
    fn from(value: &str) -> Self {
        // TODO: This allocates+copies twice.
        let value = widestring::WideString::from_str(value);
        Self::from_wide(&value)
    }
}
impl ::std::convert::From<::std::string::String> for BSTR {
    fn from(value: ::std::string::String) -> Self {
        value.as_str().into()
    }
}
impl ::std::convert::From<&::std::string::String> for BSTR {
    fn from(value: &::std::string::String) -> Self {
        value.as_str().into()
    }
}

#[cfg(windows)]
type FromWidestringError = ::std::string::FromUtf16Error;
#[cfg(not(windows))]
type FromWidestringError = widestring::FromUtf32Error;
impl<'a> ::std::convert::TryFrom<&'a BSTR> for ::std::string::String {
    type Error = FromWidestringError;
    fn try_from(value: &BSTR) -> ::std::result::Result<Self, Self::Error> {
        value.as_wide().to_string()
    }
}
impl ::std::convert::TryFrom<BSTR> for ::std::string::String {
    type Error = FromWidestringError;
    fn try_from(value: BSTR) -> ::std::result::Result<Self, Self::Error> {
        value.as_wide().to_string()
    }
}

impl ::std::default::Default for BSTR {
    fn default() -> Self {
        Self(::std::ptr::null_mut())
    }
}

impl ::std::fmt::Display for BSTR {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(&self.as_wide().to_string().unwrap())
    }
}
impl ::std::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::write!(f, "{}", self)
    }
}

impl ::std::cmp::PartialEq for BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
    }
}
impl ::std::cmp::PartialEq<::std::string::String> for BSTR {
    fn eq(&self, other: &::std::string::String) -> bool {
        self == other.as_str()
    }
}
impl ::std::cmp::PartialEq<str> for BSTR {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}
impl ::std::cmp::PartialEq<&str> for BSTR {
    fn eq(&self, other: &&str) -> bool {
        let other = widestring::WideString::from_str(other);
        self.as_wide().eq(&other)
    }
}
impl ::std::cmp::PartialEq<BSTR> for &str {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}

impl ::std::ops::Drop for BSTR {
    fn drop(&mut self) {
        #[cfg(not(windows))]
        unsafe fn SysFreeString(s: BSTR_abi) {
            ::std::boxed::Box::from_raw(s.cast::<u32>().offset(-1));
        }

        if !self.0.is_null() {
            unsafe { SysFreeString(self.abi()) };
        }
    }
}

unsafe impl ::windows::Abi for BSTR {
    type Abi = *mut widestring::WideChar;

    fn set_abi(&mut self) -> *mut *mut widestring::WideChar {
        debug_assert!(self.0.is_null());
        &mut self.0 as *mut _ as _
    }
}
#[allow(non_camel_case_types)]
pub type BSTR_abi = *mut widestring::WideChar;
