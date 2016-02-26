#![macro_use]

use libc::{ c_char, c_void };
use std::ffi::{ CString, CStr };
use std::slice;

pub fn res_to_result(res: bool) -> Result<(), ()> {
    if res { Ok(()) } else { Err(()) }
}

pub type RustStringCreator = extern fn(*const c_char) -> *mut c_char;
pub type RustU8VecCreator = extern fn(*const u8, usize) -> *mut c_void;
pub type RustStringVecCreator = extern fn(*const *const c_char, usize) -> *mut c_void;

// This is used for all functions that return a C++ string
// The argument `data` is assumed to have a final \0, it's a valid C string.
// This function needs to allocate a Rust-owned memory space to copy this string too. Here we use
// to_string_lossy ant into_owned to ensure this. Then we use CString::into_raw to get a char*
// we'll pass back to the C function, that will return it. Then in our get_as_string wrapper we'll
// transform this back into a CString using from_raw.
pub extern "C" fn rust_string_creator(data: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(data) };
    let lossy_str = c_str.to_string_lossy();
    let own_str = lossy_str.into_owned();

    return CString::new(own_str).unwrap().into_raw();
}

pub extern "C" fn rust_u8_vec_creator(data: *const u8, length: usize) -> *mut c_void {
    let rust_data = unsafe { slice::from_raw_parts(data, length) };
    let mut vec = Box::new(Vec::with_capacity(length));
    for item in rust_data {
        vec.push(item);
    }
    Box::into_raw(vec) as *mut c_void
}

pub extern "C" fn rust_string_vec_creator(data: *const *const c_char, length: usize) -> *mut c_void {
    let rust_data = unsafe { slice::from_raw_parts(data, length) };
    let mut vec = Box::new(Vec::with_capacity(length));
    for item in rust_data {
        let c_string = rust_string_creator(*item);
        vec.push(c_string);
    }
    Box::into_raw(vec) as *mut c_void
}

#[macro_export]
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
