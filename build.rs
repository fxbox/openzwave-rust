extern crate gcc;

fn main() {
    gcc::compile_library("libopenzwave-c.a", &["src/openzwave-c/options.cc"]);
}
