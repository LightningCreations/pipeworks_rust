
use crate::sys;
use crate::engine::Engine;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::ffi::c_void;
use core::mem;
use std::mem::ManuallyDrop;

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct State(sys::pw_game_state);

pub const STATE_PRIME: State = State(sys::PW_STATE_PRIME);

impl From<sys::pw_game_state> for State{
    fn from(s: sys::pw_game_state) -> Self {
        State(s)
    }
}

pub struct Game<'a>{
    ptr: NonNull<sys::pw_game>,
    fns: PhantomData<&'a mut dyn FnMut(State,&mut Engine) + Send + 'a>
}

unsafe extern"C" fn load_state<'a,F: FnMut(State,&mut Engine)+Send+'a>(state: sys::pw_game_state,engine: *mut sys::pw_engine,userdata: *mut c_void){
    let obj = &mut *(userdata.cast::<F>());
    // Yes this is technically UB.
    // Yes its necessary
    // No its not unsound, because I know what I'm doing.
    // Its safety invariant anyways, not validity. As long as engine isn't accessed off-thread,
    // or dropped.
    // That's what ManuallyDrop is for
    obj(state.into(),&mut ManuallyDrop::new(Engine::from_ptr_unchecked(engine)));
}

impl<'a> Game<'a>{
    pub fn new() -> Option<Self>{
        NonNull::new(unsafe{sys::pw_init_engine()}).map(|ptr|Self{ptr,fns: PhantomData})
    }
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_game) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),fns: PhantomData}
    }
    pub unsafe fn from_nullable_unchecked(ptr: *mut sys::pw_game) -> Option<Self>{
        NonNull::new(ptr).map(|ptr|Self{ptr,fns: PhantomData})
    }

    pub fn into_inner(self)->*mut sys::pw_game{
        let ptr = self.ptr.as_ptr();
        mem::forget(self);
        ptr
    }
    pub fn on_load_state<F: FnMut(State,&mut Engine)+Send>(&mut self,load_state: &'a mut F){
        unsafe{sys::pw_set_load_state(self.ptr.as_ptr(),&load_state::<'a,F>,(load_state as *mut _).cast())}
    }
}

impl Drop for Game<'_>{
    fn drop(&mut self) {
        unsafe{
            sys::pw_destroy_game(self.ptr.as_ptr())
        }
    }
}