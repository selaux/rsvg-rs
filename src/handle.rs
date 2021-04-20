use rsvg_sys;
use std::{ptr, str::FromStr};

use auto::Handle;
use glib::translate::*;
use glib::Error;

impl FromStr for Handle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unsafe {
            let mut error = ptr::null_mut();
            let handle =
                rsvg_sys::rsvg_handle_new_from_data(s.as_ptr() as *mut _, s.len() as _, &mut error);
            if error.is_null() {
                Ok(from_glib_full(handle))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
