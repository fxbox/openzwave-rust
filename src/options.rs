use ffi::options as extern_options;
use ffi::get_default_config_path;
use std::ffi::CString;
use ffi::utils::res_to_result;
use error::{ Result, Error };

pub struct Options {
    ptr: *mut extern_options::Options
}

impl Options {
    pub fn create(config_path: &str, user_path: &str, command_line: &str) -> Result<Options> {
        let config_path_c = if config_path.is_empty() {
            CString::new(&*get_default_config_path().to_string_lossy())
        } else {
            CString::new(config_path)
        }.unwrap();

        let user_path_c = CString::new(user_path).unwrap();
        let command_line_c = CString::new(command_line).unwrap();
        let external_options = unsafe {
            extern_options::options_create(config_path_c.as_ptr(), user_path_c.as_ptr(), command_line_c.as_ptr())
        };

        if external_options.is_null() {
            Err(Error::InitError("Could not create Options, see Openzwave logs for more information"))
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

    pub fn add_option_bool(&mut self, name: &str, value: bool) -> Result<()> {
        let name_c = CString::new(name).unwrap();
        res_to_result(
            unsafe { extern_options::options_add_option_bool(self.ptr, name_c.as_ptr(), value) }
        ).or(Err(Error::OptionsAreLocked("Options::add_option_bool")))
    }

    pub fn add_option_int(&mut self, name: &str, value: i32) -> Result<()> {
        let name_c = CString::new(name).unwrap();
        res_to_result(
            unsafe { extern_options::options_add_option_int(self.ptr, name_c.as_ptr(), value) }
        ).or(Err(Error::OptionsAreLocked("Options::add_option_int")))
    }

    pub fn add_option_string(&mut self, name: &str, value: &str, append: bool) -> Result<()> {
        let name_c = CString::new(name).unwrap();
        let value_c = CString::new(value).unwrap();
        res_to_result(
            unsafe { extern_options::options_add_option_string(self.ptr, name_c.as_ptr(), value_c.as_ptr(), append) }
        ).or(Err(Error::OptionsAreLocked("Options::add_option_string")))
    }

    pub fn lock(&mut self) -> Result<()> {
        res_to_result(unsafe { extern_options::options_lock(self.ptr) })
            .or(Err(Error::OptionsAreLocked("Options::lock")))
    }
}

impl Drop for Options {
    fn drop(&mut self) {
        res_to_result(unsafe { extern_options::options_destroy() }).unwrap();
    }
}
