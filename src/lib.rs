extern crate image;
#[macro_use]
extern crate error_chain;
extern crate regex;

mod ffi;
mod group;
mod mattex;
mod scenpar;
mod mapgen;
mod misc;

/// Used to access handle pointers across modules internally.
trait Handle<T> {
    fn handle(&self) -> *mut T;
}

pub mod easy;
pub use group::Group;
pub use mattex::{MaterialMap, TextureMap};
pub use scenpar::Scenpar;
pub use mapgen::{MapGen, MapGenHandle};
pub use misc::*;

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
            NoParameterDefs {
                description("no ParameterDefs.txt file found")
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
