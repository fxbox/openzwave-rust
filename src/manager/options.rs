#[link(name = "openzwave-c", kind = "static")]
mod extern_options {
    use libc::{size_t, c_int, c_char};

    pub enum Options {}
    extern {
        pub fn options_create(configPath: *const c_char, userPath: *const c_char, commandLine: *const c_char) -> *mut Options;
    }
}

use std::ffi::CString;

pub fn options_create(config_path: &str, user_path: &str, command_line: &str) {
    let config_path_c = CString::new(config_path).unwrap();
    let user_path_c = CString::new(user_path).unwrap();
    let command_line_c = CString::new(command_line).unwrap();
    unsafe {
        extern_options::options_create(config_path_c.as_ptr(), user_path_c.as_ptr(), command_line_c.as_ptr());
    }
}
