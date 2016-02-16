#![macro_use]

use libc::c_char;
use std::ffi::{ CString, CStr };


pub fn res_to_result(res: bool) -> Result<(), ()> {
    if res { Ok(()) } else { Err(()) }
}

// This is used for all functions that return a C++ string
// The argument `data` is assumed to have a final \0, it's a valid C string.
// This function needs to allocate a Rust-owned memory space to copy this string too. Here we use
// to_string_lossy ant into_owned to ensure this. Then we use CString::into_raw to get a char*
// we'll pass back to the C function, that will return it. Then in our get_as_string wrapper we'll
// transform this back into a CString using from_raw.
pub extern "C" fn get_string_callback(data: *const c_char) -> *const c_char {
    let str = unsafe { CStr::from_ptr(data) }.to_string_lossy().into_owned();

    return CString::new(str).unwrap().into_raw();
}

macro_rules! c_like_enum {
    ( $name: ident { $($variant: ident = $value: expr),+ } ) => {
        #[derive(Debug)]
        pub enum $name {
            $($variant = $value,)+
        }

        impl $name {
            pub fn from_u8(value: u8) -> Option<$name> {
                match value {
                    $($value => Some($name::$variant),)+
                    _ => None
                }
            }
        }
    }
}
