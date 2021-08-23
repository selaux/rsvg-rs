use glib::translate::*;
use glib::Error;
use std::{ptr, str::FromStr};

use crate::auto::Handle;

impl FromStr for Handle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unsafe {
            let mut error = ptr::null_mut();
            let handle =
                ffi::rsvg_handle_new_from_data(s.as_ptr() as *mut _, s.len() as _, &mut error);
            if error.is_null() {
                Ok(from_glib_full(handle))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
