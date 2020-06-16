
use crate::sys;
use crate::engine::Engine;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::ffi::c_void;
use core::mem;
use std::mem::ManuallyDrop;
use std::pin::Pin;

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct State(sys::pw_game_state);

pub const STATE_PRIME: State = State(sys::STATE_PRIME as u8);

impl From<sys::pw_game_state> for State{
    fn from(s: sys::pw_game_state) -> Self {
        State(s)
    }
}


unsafe extern"C" fn drop_boxed<F>(v: *mut c_void){
    drop(Box::<F>::from_raw(v.cast()));
}


pub struct Game<'a>{
    ptr: NonNull<sys::pw_game>,
    fn_phantom: PhantomData<(dyn FnMut(State,&mut Engine) + Send +'a)>,
}

unsafe extern"C" fn load_state<'a,F: FnMut(State,Pin<&mut Engine>)+Send+'a>(state: sys::pw_game_state,engine: *mut sys::pw_engine,userdata: *mut c_void){
    let obj = &mut *(userdata.cast::<F>());
    // Yes this is technically UB.
    // Yes its necessary
    // No its not unsound, because I know what I'm doing.
    // Its safety invariant anyways, not validity. As long as engine isn't accessed off-thread,
    // or dropped.
    // That's what ManuallyDrop is for
    obj(state.into(),Pin::new_unchecked(&mut ManuallyDrop::new(Engine::from_ptr_unchecked(engine))));
}

impl Game<'_>{
    pub fn new() -> Option<Self>{
        NonNull::new(unsafe{sys::pw_init_game()}).map(|ptr|Self{ptr,fn_phantom: PhantomData})
    }
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_game) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),fn_phantom: PhantomData}
    }
    pub unsafe fn from_nullable_unchecked(ptr: *mut sys::pw_game) -> Option<Self>{
        NonNull::new(ptr).map(|ptr|Self{ptr,fn_phantom: PhantomData})
    }

    pub fn into_inner(self)->*mut sys::pw_game{
        let ptr = self.ptr.as_ptr();
        mem::forget(self);
        ptr
    }
}

impl<'a> Game<'a>{
    pub fn on_load_state<F: FnMut(State,Pin<&mut Engine>)+Send+'a>(&mut self,load_state:F){
        let load_state = Box::new(load_state);
        unsafe{sys::pw_set_load_state_cleanup(self.ptr.as_ptr(),Some(self::load_state::<'a,F>),
                                              Box::into_raw(load_state).cast(),Some(drop_boxed::<F>))}
    }
}

impl Drop for Game<'_>{
    fn drop(&mut self) {
        unsafe{
            sys::pw_destroy_game(self.ptr.as_ptr())
        }
    }
}