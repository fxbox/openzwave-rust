#[link(name = "openzwave-c", kind = "static")]
mod extern_options {
    use libc::c_char;

    pub enum Options {}
    extern {
        pub fn options_create(configPath: *const c_char, userPath: *const c_char, commandLine: *const c_char) -> *mut Options;
        pub fn options_get() -> *mut Options;
        pub fn options_lock(options: *mut Options) -> bool;
        pub fn options_destroy() -> bool;
    }
}

use std::ffi::CString;
use utils::res_to_result;

pub struct Options {
    ptr: *mut extern_options::Options
}

impl Options {
    pub fn create(config_path: &str, user_path: &str, command_line: &str) -> Result<Options, ()> {
        let config_path_c = CString::new(config_path).unwrap();
        let user_path_c = CString::new(user_path).unwrap();
        let command_line_c = CString::new(command_line).unwrap();
        let external_options = unsafe {
            extern_options::options_create(config_path_c.as_ptr(), user_path_c.as_ptr(), command_line_c.as_ptr())
        };

        if external_options.is_null() {
            Err(())
        } else {
            Ok(Options { ptr: external_options })
        }
    }

    pub fn get() -> Option<Options> {
        let external_options = unsafe { extern_options::options_get() };
        if external_options.is_null() {
            None
        } else {
            Some(Options { ptr: external_options })
        }
    }

    pub fn lock(&mut self) -> Result<(), ()> {
        res_to_result(unsafe { extern_options::options_lock(self.ptr) })
    }
}

impl Drop for Options {
    fn drop(&mut self) {
        res_to_result(unsafe { extern_options::options_destroy() }).unwrap();
    }
}
