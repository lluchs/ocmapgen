use ffi::*;
use std::ffi::CStr;

/// Seeds the RNG used for map generation.
pub fn seed_rng(seed: u32) {
    unsafe {
        c4_random_handle_seed(seed);
    }
}

/// Returns the version of the linked OpenClonk library.
pub fn openclonk_version() -> String {
    unsafe {
        let version = c4_version_get();
        CStr::from_ptr(version).to_string_lossy().into_owned()
    }
}
