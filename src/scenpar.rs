use ffi::*;
use errors::*;
use super::Handle;
use group::Group;

use std::ffi::{CStr, CString};

pub struct Scenpar {
    handle: *mut C4ScenparHandle,
}

impl Scenpar {
    pub fn new() -> Scenpar {
        unsafe {
            Scenpar {
                handle: c4_scenpar_handle_new(),
            }
        }
    }

    /// Tries to load ParameterDefs.txt from the given group.
    pub fn load(&mut self, group: &Group) -> Result<()> {
        unsafe {
            if c4_scenpar_handle_load(self.handle, group.handle()) {
                Ok(())
            } else {
                bail!("couldn't load scenario parameters: {}",
                      CStr::from_ptr(c4_log_handle_get_log_messages())
                            .to_string_lossy());
            }
        }
    }

    pub fn get_value_by_id(&self, id: &str, default_value: i32) -> i32 {
        unsafe {
            c4_scenpar_handle_get_value_by_id(self.handle,
                CString::new(id).unwrap().as_ptr(),
                default_value)
        }
    }

    pub fn set_value(&mut self, id: &str, value: i32, only_if_larger: bool) {
        unsafe {
            c4_scenpar_handle_set_value(self.handle,
                CString::new(id).unwrap().as_ptr(),
                value, only_if_larger);
        }
    }
}

impl Drop for Scenpar {
    fn drop(&mut self) {
        unsafe {
            c4_scenpar_handle_free(self.handle);
        }
    }
}

impl Handle<C4ScenparHandle> for Scenpar {
    fn handle(&self) -> *mut C4ScenparHandle {
        self.handle
    }
}
