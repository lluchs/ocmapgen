# Use rust-bindgen to generate bindings.
#
# As rust-bindgen has system dependencies on clang, the generated file is
# checked into git and not created in build.rs.

src/ffi.rs: src/cpp-handles/*.h
	bindgen --whitelist-function 'c4_.*' --raw-line '#![allow(dead_code)]' -o$@ src/cpp-handles/bindgen.h
