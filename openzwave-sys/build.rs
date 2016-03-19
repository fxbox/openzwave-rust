extern crate gcc;
extern crate pkg_config;

fn main() {
    let libopenzwave = pkg_config::Config::new()
        .atleast_version("1.4").probe("libopenzwave").unwrap();
    let mut c = gcc::Config::new();
    for inc in libopenzwave.include_paths {
        // workaround for https://github.com/OpenZWave/open-zwave/issues/771
        let mut parent = inc.clone();
        parent.pop();

        c.include(inc);
        c.include(parent);
    }

    c.file("openzwave-c/options.cc")
     .file("openzwave-c/manager.cc")
     .file("openzwave-c/notification.cc")
     .file("openzwave-c/value_classes/value_id.cc")
     .cpp(true)
     .flag("-std=c++11")  // to iterate with ranges
     .flag("-pedantic").flag("-Wall").flag("-Wextra")
     .compile("libopenzwave-c.a");
    println!("cargo:rustc-flags=-l openzwave");
}
