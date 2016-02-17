Open-Zwave wrapper for Rust
===========================

Installation
------------

Simply using the crate should work.

### fatal error: Defs.h: No such file or directory

If you have your openzwave header files in an unusual place
(`/usr/include/openzwave` and `/usr/local/include/openzwave` are hardcoded in
`build.rs`), you can use the environment variable `CPATH`
to specify it.

### Your libopenzwave.so is not in a standard path.

You should likely add the path where it's installed to your `/etc/ld.so.conf`.

