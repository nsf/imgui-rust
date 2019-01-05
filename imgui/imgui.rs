mod auto;

pub use crate::auto::types_used::*;
pub use crate::auto::*;
use nsf_imgui_raw::*;
use lazy_static::lazy_static;
use std::rc::Rc;
use std::sync::Mutex;

lazy_static! {
    static ref IMGUI_INSTANCE_ALIVE: Mutex<bool> = Mutex::new(false);
}

pub struct ImGuiOwner;

impl Drop for ImGuiOwner {
    fn drop(&mut self) {
        let mut is_alive = IMGUI_INSTANCE_ALIVE.lock().unwrap();
        unsafe {
            igDestroyContext(igGetCurrentContext());
        }
        *is_alive = false;
    }
}

#[derive(Clone)]
pub struct ImGui {
    imgui_owner: Rc<ImGuiOwner>,
}

impl ImGui {
    pub fn new() -> Result<ImGui, String> {
        let mut is_alive = IMGUI_INSTANCE_ALIVE.lock().unwrap();
        if *is_alive {
            Err("Cannot initialize `ImGui` more than once at a time.".to_owned())
        } else {
            unsafe { igCreateContext(std::ptr::null_mut()) };
            *is_alive = true;
            Ok(ImGui {
                imgui_owner: Rc::new(ImGuiOwner),
            })
        }
    }
}

#[macro_export]
macro_rules! cstr_ptr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char] as *const ::std::os::raw::c_char
    };
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        ::std::ffi::CStr::from_ptr(cstr_ptr!($s))
    );
    ($s:expr, $($arg:expr)*) => (
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(&format!(concat!($s, "\0"), $($arg)*).into_bytes())
    );
}
