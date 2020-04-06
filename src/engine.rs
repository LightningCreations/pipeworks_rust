use crate::sys;
use std::ptr::NonNull;
use std::marker::{PhantomPinned, PhantomData};
use core::mem;
use crate::game::Game;

pub struct Engine<'a>{
    ptr: NonNull<sys::pw_engine>,
    game: PhantomData<Option<Game<'a>>>,
    things: PhantomData<Option<Thing>>
}

impl<'a> Engine<'a>{
    pub fn new() -> Option<Self>{
        NonNull::new(sys::pw_init_engine()).map(|ptr|Self{ptr,game:PhantomData,things: PhantomData})
    }
    ///
    /// Constructs a new Engine from a raw pointer to a sys::pw_engine
    /// The constructor is owning, there must not be more than one Engine object holding the same pointer.
    ///
    /// ptr shall have been obtained from a call to Engine::into_inner(),
    ///  or sys::pw_init_engine(), and must not be destroyed
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_engine) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),game: PhantomData,things: PhantomData}
    }

    pub fn into_inner(self) -> *mut sys::pw_engine{
        let ptr = self.ptr.as_ptr();
        mem::forget(self);
        ptr
    }

    pub fn set_game(&mut self,game: Game<'a>){
        sys::pw_set_game(self.ptr.as_ptr(),game.into_inner());
    }

    pub fn take_game(&mut self)-> Option<Game<'a>>{
        unsafe{Game::from_nullable_unchecked(sys::pw_engine_release_game(self.ptr.as_ptr()))}
    }
}


///
/// Cleans up the engine
/// Dropping an Engine
impl Drop for Engine<'_>{
    fn drop(&mut self) {
        unsafe{
            sys::pw_engine_destroy(self.ptr.as_ptr())
        }
    }
}