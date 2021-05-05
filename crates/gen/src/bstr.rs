use super::*;

pub fn gen_bstr() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::cmp::Eq)]
        /// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr#remarks
        /// Uses [`::windows::widestring::UStr`] and not [`::windows::widestring::UCstr`], the latter checks for internal nulls.
        pub struct BSTR(pub *mut ::windows::widestring::WideChar);
        impl BSTR {
            /// Create an empty [`BSTR`].
            ///
            /// This function does not allocate memory.
            pub fn new() -> Self {
                Self(std::ptr::null_mut())
            }

            /// Returns [`true`] if the string is empty.
            pub fn is_empty(&self) -> bool {
                // TODO: Should possibly also check length!
                self.0.is_null()
            }

            /// Returns the length of the string.
            pub fn len(&self) -> usize {
                if self.is_empty() {
                    return 0;
                }

                unsafe { SysStringLen(self) as usize }
            }

            /// Create a [`BSTR`] from a slice of 16-bit characters.
            fn from_wide(value: &::windows::widestring::WideStr) -> Self {
                if value.len() == 0 {
                    return Self(::std::ptr::null_mut());
                }
                unsafe {
                    SysAllocStringLen(
                        PWSTR(value.as_ptr() as _),
                        value.len() as u32,
                    )
                }
            }

            /// Get the string as 16-bit characters.
            fn as_wide(&self) -> &::windows::widestring::WideStr {
                if self.0.is_null() {
                    // `UStr` unlike `UCStr` doesn't implement an empty-string default yet
                    ::windows::widestring::WideStr::from_slice(&[])
                } else {
                    unsafe { ::windows::widestring::WideStr::from_ptr(self.0, self.len()) }
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
                let value = ::windows::widestring::WideString::from_str(value);
                Self::from_wide(&value)
            }
        }
        impl ::std::convert::From<::std::string::String> for BSTR {
            fn from(value: ::std::string::String) -> Self {
                value.as_str().into()
            }
        }
        impl  ::std::convert::From<&::std::string::String> for BSTR {
            fn from(value: &::std::string::String) -> Self {
                value.as_str().into()
            }
        }

        #[cfg(windows)]
        type FromWidestringError = ::std::string::FromUtf16Error;
        #[cfg(not(windows))]
        type FromWidestringError = ::windows::widestring::FromUtf32Error;
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
                let other = ::windows::widestring::WideString::from_str(other);
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
                if !self.0.is_null() {
                    unsafe { SysFreeString(self as &Self) }
                }
            }
        }

        unsafe impl ::windows::Abi for BSTR {
            type Abi = ::std::mem::ManuallyDrop<Self>;
            type DefaultType = Self;
        }
        pub type BSTR_abi = *mut ::windows::widestring::WideChar;
    }
}
