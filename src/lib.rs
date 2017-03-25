//extern crate libc;
extern crate image;
#[macro_use]
extern crate error_chain;

mod ffi;
mod group;
mod mattex;
mod mapgen;

/// Used to access handle pointers across modules internally.
trait Handle<T> {
    fn handle(&self) -> *mut T;
}

pub use group::Group;
pub use mattex::{MaterialMap, TextureMap};
pub use mapgen::MapGen;

mod errors {
    error_chain! {
        errors {
            Group(err: String) {
                description("c4group error")
                display("c4group: {}", err)
            }
            NothingLoaded {
                description("texture/material loading failed")
            }
            MapGen(err: String) {
                description("map generation error")
                display("{}", err)
            }
        }
    }
}

pub use errors::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
