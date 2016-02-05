extern crate gcc;

fn main() {
    let mut c = gcc::Config::new();
    c.file("src/openzwave-c/options.cc")
     .cpp(true)
     .compile("libopenzwave-c.a");
    println!("cargo:rustc-flags=-l openzwave");
    //println!("cargo:rustc-link-lib=static=foo")
}
