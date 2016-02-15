use libc::c_char;

pub enum Options {}
extern {
    pub fn options_create(configPath: *const c_char, userPath: *const c_char, commandLine: *const c_char) -> *mut Options;
    pub fn options_get() -> *mut Options;
    pub fn options_lock(options: *mut Options) -> bool;
    pub fn options_destroy() -> bool;
}

