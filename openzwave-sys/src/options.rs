use libc::c_char;

pub enum Options {}
extern {
    pub fn options_create(configPath: *const c_char, userPath: *const c_char, commandLine: *const c_char) -> *mut Options;
    pub fn options_get() -> *mut Options;
    pub fn options_lock(options: *mut Options) -> bool;
    pub fn options_add_option_bool(options: *mut Options, name: *const c_char, value: bool) -> bool;
    pub fn options_add_option_int(options: *mut Options, name: *const c_char, value: i32) -> bool;
    pub fn options_add_option_string(options: *mut Options, name: *const c_char, value: *const c_char, append: bool) -> bool;
    pub fn options_destroy() -> bool;
}

