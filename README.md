ocmapgen
========

Rust bindings for the OpenClonk dynamic map generator based on mape.

![Acid Gold Mine map generated using ocmapgen](https://i.imgur.com/EJdSxQz.png)

Compiling
---------

In addition to the tools required to build OpenClonk, get Rust via [rustup](https://www.rustup.rs/).
From OpenClonk's dependencies, only the zlib library and glew headers are necessary.

Run (on Linux, but may work similarly on Windows as well I guess):

    git clone --recursive https://github.com/lluchs/ocmapgen
	cd ocmapgen/ocmapgen-bin
    cargo build --release

The binary can then be copied from `target/release/ocmapgen`.
 
To cross-compile to Windows from Arch Linux, I installed mingw-w64-{gcc,cmake,zlib,glew} and run:

    CMAKE_TOOLCHAIN_FILE=/usr/share/mingw/toolchain-x86_64-w64-mingw32.cmake cargo build --release --target x86_64-pc-windows-gnu

Command-Line Usage
------------------

Put ocmapgen into the OC game file directory (“planet”), then run

    ./ocmapgen Map.c Map.png

Select output image size with `--width` and `--height`.
