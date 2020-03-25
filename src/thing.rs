use crate::sys;
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Thing<'a>{
    ptr: NonNull<sys::pw_thing>,
    lifetime: PhantomData<&'a ()>
}

impl<'a> Thing<'a>{
    pub fn new() -> Option<Self>{
        NonNull::new(unsafe{sys::pw_init_thing()}).map(|ptr|Self{ptr,lifetime: PhantomData})
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_thing) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),lifetime: PhantomData}
    }

    pub fn into_inner(self) -> *mut sys::pw_thing{
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }
}

impl<'a> Drop for Thing<'a>{
    fn drop(&mut self) {
        unsafe{
            sys::pw_destroy_thing(self.ptr.as_ptr())
        }
    }
}

unsafe impl<'a> Send for Thing<'a>{}

#[repr(C)]
pub struct ThingRef<'a>{
    ptr: NonNull<sys::pw_thing>,
    r#ref: PhantomData<&'a mut Thing<'a>>
}

impl<'a> ThingRef<'a>{
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_thing) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),r#ref: PhantomData}
    }

    // Shutup Clippy
    // Box does this
    pub fn into_inner(thingref{ptr,r#ref}: Self) -> *mut sys::pw_thing{
        ptr.as_ptr()
    }
}

impl<'a> Deref<Target> for ThingRef<'a>{
    type Target = Thing<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe{std::mem::transmute(self)}
    }
}

impl<'a> DerefMut<Target> for ThingRef<'a>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{std::mem::transmute(self)}
    }
}