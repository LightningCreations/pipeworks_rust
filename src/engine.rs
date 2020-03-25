use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::sys;
use crate::game::Game;
use crate::thing::{Thing, ThingRef};

pub struct Engine<'a>{
    ptr: NonNull<sys::pw_engine>,
    lifetime: PhantomData<crate::game::Game<'a>>,
    things: PhantomData<crate::thing::Thing<'a>>
}

impl<'a> Drop for Engine<'a>{
    fn drop(&mut self) {
        unsafe{
            sys::pw_destroy_engine(self.ptr.as_ptr());
        }
    }
}


impl<'a> Engine<'a>{
    pub fn new() -> Option<Self>{
        NonNull::new(unsafe{sys::pw_init_engine()})
            .map(|ptr|Self{ptr,lifetime: PhantomData,things: PhantomData})
    }
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_engine) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),lifetime: PhantomData,things: PhantomData}
    }

    pub fn stop(self){
        unsafe{
            sys::pw_stop(self.ptr.as_ptr())
        }
    }

    pub fn join(self){
        unsafe{
            sys::pw_join(self.ptr.as_ptr())
        }
    }
    pub fn into_inner(self) -> *mut sys::pw_engine{
        let ret = self.ptr.as_ptr();
        std::mem::forget(self);
        ret
    }
    pub fn set_game(&mut self,game: Game<'a>) {
        unsafe{
            sys::pw_set_game(self.ptr.as_ptr(),game.into_inner())
        }
    }

    pub fn add_thing(&mut self,thing: Thing<'a>) -> ThingRef<'a> {
        unsafe{
            let ptr = thing.into_inner();
            sys::pw_engine_add_thing(self.ptr.as_ptr(),ptr);
            ThingRef::from_ptr_unchecked(ptr)
        }
    }

    pub fn clear_things(&mut self){
        unsafe{
            sys::pw_engine_clear_things(self.ptr.as_ptr())
        }
    }
}

unsafe impl<'a> Send for Engine<'a>{}