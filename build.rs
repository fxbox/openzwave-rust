extern crate gcc;

fn main() {
    let mut c = gcc::Config::new();
    c.file("src/openzwave-c/options.cc")
     .file("src/openzwave-c/manager.cc")
     .file("src/openzwave-c/notification.cc")
     .file("src/openzwave-c/value_classes/value_id.cc")
     .include("/usr/include/openzwave") // workaround for https://github.com/OpenZWave/open-zwave/issues/771
     .cpp(true)
     .compile("libopenzwave-c.a");
    println!("cargo:rustc-flags=-l openzwave");
    //println!("cargo:rustc-link-lib=static=foo")
}
