extern crate gcc;

fn main() {
    let mut c = gcc::Config::new();
    c.file("openzwave-c/options.cc")
     .file("openzwave-c/manager.cc")
     .file("openzwave-c/notification.cc")
     .file("openzwave-c/value_classes/value_id.cc")
     .include("/usr/include/openzwave") // workaround for https://github.com/OpenZWave/open-zwave/issues/771
     .cpp(true)
     .compile("libopenzwave-c.a");
    println!("cargo:rustc-flags=-l openzwave");
    //println!("cargo:rustc-link-lib=static=foo")
}
