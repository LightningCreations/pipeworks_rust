use crate::sys;
use std::ptr::NonNull;
use std::marker::PhantomData;

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

