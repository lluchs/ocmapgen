use crate::ffi::*;
use crate::errors::*;
use crate::Handle;

use std::os::raw::{c_void, c_char};
use std::ffi::{CStr, CString};
use std::ptr;

use error_chain::bail;

pub struct Group {
    handle: *mut C4GroupHandle,
}

macro_rules! group_error {
    ($handle:expr) => {
        bail!(ErrorKind::Group(CStr::from_ptr(c4_group_handle_get_error($handle)).to_string_lossy().into_owned()));
    };
}

impl Group {
    fn new() -> Group {
        unsafe {
            Group {
                handle: c4_group_handle_new(),
            }
        }
    }

    pub fn open(path: &str, create: bool) -> Result<Group> {
        let group = Group::new();
        unsafe {
            if c4_group_handle_open(group.handle, CString::new(path).unwrap().as_ptr(), create) {
                Ok(group)
            } else {
                group_error!(group.handle);
            }
        }
    }

    pub fn open_as_child(mother: &Group, name: &str, exclusive: bool, create: bool) -> Result<Group> {
        let group = Group::new();
        unsafe {
            if c4_group_handle_open_as_child(group.handle, mother.handle, CString::new(name).unwrap().as_ptr(), exclusive, create) {
                Ok(group)
            } else {
                group_error!(group.handle);
            }
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            CStr::from_ptr(c4_group_handle_get_name(self.handle))
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn full_name(&self) -> String {
        unsafe {
            CStr::from_ptr(c4_group_handle_get_full_name(self.handle))
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn rewind(&mut self) {
        unsafe {
            c4_group_handle_reset_search(self.handle);
        }
    }

    pub fn load_entry(&self, name: &str) -> Result<Vec<u8>> {
        unsafe {
            let entry = CString::new(name.to_string()).unwrap();
            let mut size: usize = 0;
            if !c4_group_handle_access_entry(self.handle, entry.as_ptr(), &mut size, ptr::null_mut(), false) {
                group_error!(self.handle);
            }
            let mut data: Vec<u8> = vec![0; size];
            if !c4_group_handle_read(self.handle, data.as_mut_ptr() as *mut c_void, size) {
                group_error!(self.handle);
            }
            Ok(data)
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            c4_group_handle_free(self.handle);
        }
    }
}

impl Handle<C4GroupHandle> for Group {
    fn handle(&self) -> *mut C4GroupHandle {
        self.handle
    }
}


impl Iterator for Group {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut buf: Vec<u8> = vec![0; 512];
            let wildcard = CString::new("*").unwrap();
            if c4_group_handle_find_next_entry(self.handle, wildcard.as_ptr(), ptr::null_mut(), buf.as_mut_slice().as_mut_ptr() as *mut c_char, false) {
                if let Some(pos) = buf.iter().position(|&b| b == 0) {
                    buf.truncate(pos + 1);
                    let name = CStr::from_bytes_with_nul(&buf).unwrap().to_string_lossy().into_owned();
                    Some(name)
                } else {
                    panic!("string not null-terminated");
                }
            } else {
                None
            }
        }
    }
}
