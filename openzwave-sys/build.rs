extern crate gcc;
use std::env;

fn main() {
    let mut c = gcc::Config::new();
    let config = c.file("openzwave-c/options.cc")
     .file("openzwave-c/manager.cc")
     .file("openzwave-c/notification.cc")
     .file("openzwave-c/value_classes/value_id.cc")
     .include("/usr/include/openzwave") // workaround for https://github.com/OpenZWave/open-zwave/issues/771
     .include("/usr/local/include/openzwave")
     .cpp(true);

    if let Ok(path) = env::var("OPENZWAVE_INCLUDE_PATH") {
        config.include(path);
    }

    config.compile("libopenzwave-c.a");
    println!("cargo:rustc-flags=-l openzwave");
    //println!("cargo:rustc-link-lib=static=foo")
}
