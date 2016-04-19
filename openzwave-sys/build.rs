extern crate gcc;
use std::process::Command;
use std::env;

#[cfg(target_os = "linux")]
fn target_specific_work(_: &str) {
    println!("cargo:rustc-link-lib=udev");
    println!("cargo:rustc-link-lib=static=openzwave");
}

#[cfg(target_os = "macos")]
fn target_specific_work(openzwave_build_dir: &str) {
    // The .a is a universal (fat) binary, so let's convert it to a thin binary
    // There is no easy way to disable the fat binary generation in open-zwave:
    // https://github.com/OpenZWave/open-zwave/issues/814
    let exit_code = Command::new("lipo")
        .current_dir(openzwave_build_dir)
        .arg("-thin").arg("x86_64")
        .arg("-output").arg("libopenzwave-thin.a")
        .arg("libopenzwave.a")
        .status()
        .unwrap();

    if !exit_code.success() {
        panic!("Could not extract a thin library from the fat binary.");
    }

    println!("cargo:rustc-link-lib=static=openzwave-thin");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

#[cfg(target_os = "freebsd")]
fn target_specific_work(_: &str) {
    println!("cargo:rustc-link-lib=usb");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=static=openzwave");
}

fn make(output: &str) {
    let exit_code = Command::new("make")
        .arg(format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .env("top_builddir", output)
        .current_dir("open-zwave")
        .status().unwrap();

    if !exit_code.success() {
        panic!("Could not build the open-zwave library.");
    }
}

fn main() {
    let openzwave_build_dir = format!("{}/{}", env::var("OUT_DIR").unwrap(), "open-zwave");
    make(&openzwave_build_dir);

    // Different platforms need some different work and linking parameters
    target_specific_work(&openzwave_build_dir);

    // then build our thin wrapper
    let mut c = gcc::Config::new();
    c.file("openzwave-c/options.cc")
     .file("openzwave-c/manager.cc")
     .file("openzwave-c/notification.cc")
     .file("openzwave-c/value_classes/value_id.cc")
     .cpp(true)
     .flag("-std=c++11") // to iterate with ranges
     .include("open-zwave/cpp/src")
     .compile("libopenzwave-c.a");

    println!("cargo:rustc-link-search=native={}", openzwave_build_dir);
}
