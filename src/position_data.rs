use glib::translate::*;
use rsvg_sys;
use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct PositionData {
    pub x: i32,
    pub y: i32,
    pub em: f64,
    pub ex: f64,
}

impl PositionData {
    pub fn new(x: i32, y: i32, em: f64, ex: f64) -> PositionData {
        PositionData { x, y, em, ex }
    }
}

#[doc(hidden)]
#[allow(clippy::uninit_assumed_init)]
impl Uninitialized for PositionData {
    #[inline]
    unsafe fn uninitialized() -> Self {
        MaybeUninit::<Self>::uninit().assume_init()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const rsvg_sys::RsvgPositionData> for PositionData {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const rsvg_sys::RsvgPositionData, Self> {
        let ptr: *const PositionData = &*self;
        Stash(ptr as *const rsvg_sys::RsvgPositionData, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut rsvg_sys::RsvgPositionData> for PositionData {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut rsvg_sys::RsvgPositionData, Self> {
        let ptr: *mut PositionData = &mut *self;
        StashMut(ptr as *mut rsvg_sys::RsvgPositionData, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const rsvg_sys::RsvgPositionData> for PositionData {
    unsafe fn from_glib_none(ptr: *const rsvg_sys::RsvgPositionData) -> Self {
        *(ptr as *const PositionData)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut rsvg_sys::RsvgPositionData> for PositionData {
    unsafe fn from_glib_none(ptr: *mut rsvg_sys::RsvgPositionData) -> Self {
        *(ptr as *mut PositionData)
    }
}
