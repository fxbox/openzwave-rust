Open-Zwave wrapper for Rust
===========================

Installation
------------

Simply using the crate should work. However if openzwave's includes are not
located in `/usr/include/openzwave` or `/usr/local/include/openzwave` you'll
need to use the environment variable `OPENZWAVE_INCLUDE_PATH` to configure it.
This is because of [an issue in OpenZWave](https://github.com/OpenZWave/open-zwave/issues/771)
they don't want to fix.

