extern crate libc;

pub mod utils;
pub mod manager;
pub mod options;
pub mod notification;
pub mod value_classes;

use std::path::PathBuf;

pub fn get_default_config_path() -> PathBuf {
    let installed_path = env!("CARGO_MANIFEST_DIR");
    let mut installed_path = PathBuf::from(installed_path);
    installed_path.push("open-zwave");
    installed_path.push("config");
    installed_path
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
