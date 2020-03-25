use crate::sys;
use std::ptr::NonNull;
use std::marker::PhantomData;
use crate::engine::Engine;

pub struct State(sys::pw_game_state);

pub const STATE_PRIME: State = State(sys::STATE_PRIME as sys::pw_game_state);

pub struct Game<'a>{
    ptr: NonNull<sys::pw_game>,
    lifetime: PhantomData<&'a (dyn FnMut(&mut Engine<'a>,State)+Sync+Send+'a)>
}

impl<'a> Drop for Game<'a>{
    fn drop(&mut self) {
        unsafe{
            sys::pw_destroy_game(self.ptr.as_ptr())
        }
    }
}

unsafe extern"C" fn load_state_callback<'a,F: FnMut(&mut Engine<'a>,State) + Send + Sync + 'a>(state: sys::pw_game_state,engine: *mut sys::pw_engine,userdata: *mut std::ffi::c_void){
    (&mut *userdata.cast::<F>())(&mut Engine::from_ptr_unchecked(engine),State(state))
}

impl<'a> Game<'a>{
    pub fn new() -> Option<Game<'a>>{
        NonNull::new(unsafe{sys::pw_init_game()})
            .map(|ptr|Self{ptr,lifetime: PhantomData})
    }
    pub unsafe fn from_ptr_unchecked(ptr: *mut sys::pw_game) -> Self{
        Self{ptr: NonNull::new_unchecked(ptr),lifetime: PhantomData}
    }

    pub fn into_inner(self) -> *mut sys::pw_game{
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }

    pub fn set_load_state<F: FnMut(&mut Engine<'a>,State) + Send + Sync + 'a>(&mut self,callback: &'a mut F){
        unsafe{
            sys::pw_set_load_state(self.ptr.as_ptr(),Some(load_state_callback::<'a,F>),(callback as *mut F).cast());
        }
    }
}

unsafe impl Send for Game<'_>{}
