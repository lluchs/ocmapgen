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

pub mod easy;
pub use group::Group;
pub use mattex::{MaterialMap, TextureMap};
pub use mapgen::MapGen;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
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
            MapTypeDetectionFailed(name: String) {
                description("couldn't autodetect map type")
                display("file '{}' has neither Landscape.txt nor Map.c file type", name)
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
