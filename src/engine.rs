use crate::sys;
use std::ptr::NonNull;
use std::marker::{PhantomPinned, PhantomData};
use core::mem;
use crate::game::Game;
use std::pin::Pin;

pub struct Engine<'a>{
    ptr: NonNull<sys::pw_engine>,
    game: PhantomData<Option<Game<'a>>>,
    // Impl Note: forgetting a pinned Engine is fine, however this is to prevent apis that recieve a game from
    pinned: PhantomPinned
}

impl<'a> Engine<'a>{
    pub fn new() -> Option<Self>{
        NonNull::new(unsafe{sys::pw_init_engine()}).map(|ptr|Self{ptr,game:PhantomData,pinned: PhantomPinned})
    }
    ///
    /// Constructs a new Engine from a raw pointer to a sys::pw_engine
    /// The constructor is owning, there must not be more than one Engine object holding the same pointer.
    ///
    /// ptr shall have been obtained from a call to Engine::into_inner(),
    ///  or sys::pw_init_engine(), and must not be destroyed
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_engine) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),game: PhantomData,pinned: PhantomPinned}
    }


    pub fn set_game(self: Pin<&mut Self>,game: Game<'a>){
        unsafe{sys::pw_set_game(self.ptr.as_ptr(),game.into_inner())}
    }


    pub fn take_game(self: Pin<&mut Self>) -> Option<Game<'a>>{
        unsafe{Game::from_nullable_unchecked(sys::pw_engine_release_game(self.ptr.as_ptr()))}
    }
}

macro_rules! create_engine_ref{
    () => {
        let mut _engine = Engine::new();
        // SAFETY:
        // _engine cannot be accessed outside of the macro, because of hygine. 
        unsafe{Pin::new_unchecked(&mut _engine)}
    }
}

unsafe impl Send for Engine<'_>{}
unsafe impl Sync for Engine<'_>{}

///
/// Cleans up the engine
/// Dropping an Engine
impl Drop for Engine<'_>{
    fn drop(&mut self) {
        unsafe{
            /// SAFETY:
            /// This is `drop`, so we are never moved from. 
            drop(Pin::new_unchecked(&mut *self).take_game());
            sys::pw_destroy_engine(self.ptr.as_ptr())
        }
    }
}
