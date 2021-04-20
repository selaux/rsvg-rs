use rsvg_sys;
use std::ptr;

use glib::Error;
use glib::translate::*;
use auto::Handle;

impl Handle {
    pub fn from_str(data: &str) -> Result<Handle, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let handle = rsvg_sys::rsvg_handle_new_from_data(data.as_ptr() as *mut _, data.len() as _, &mut error);
            if error.is_null() { Ok(from_glib_full(handle)) } else { Err(from_glib_full(error)) }
        }
    }
}