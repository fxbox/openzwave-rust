extern crate gcc;
use std::process::Command;
use std::env;

#[cfg(target_os = "linux")]
fn target_specific_work() {
    println!("cargo:rustc-link-lib=udev");
    println!("cargo:rustc-link-lib=static=openzwave");
}

#[cfg(target_os = "macos")]
fn target_specific_work() {
    // The .a is a universal (fat) binary, so let's convert it to a thin binary
    // There is no easy way to disable the fat binary generation in open-zwave:
    // https://github.com/OpenZWave/open-zwave/issues/814
    let exit_code = Command::new("lipo")
        .current_dir("open-zwave")
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
fn target_specific_work() {
    println!("cargo:rustc-link-lib=usb");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=static=openzwave");
}

fn make() {
    let exit_code = Command::new("make")
        .arg("-j4")
        .current_dir("open-zwave")
        .status().unwrap();

    if !exit_code.success() {
        panic!("Could not build the open-zwave library.");
    }
}

fn main() {
    make();

    // Different platforms need some different work and linking parameters
    target_specific_work();

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

    let this_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/open-zwave", this_dir);
}
