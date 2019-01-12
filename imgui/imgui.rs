mod auto;

pub use crate::auto::types_used::*;
pub use crate::auto::*;
use lazy_static::lazy_static;
use nsf_imgui_raw::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
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

    extern "C" fn input_text_callback(data: *mut ImGuiInputTextCallbackData) -> c_int {
        unsafe {
            let s = (*data).user_data as *mut String;
            if (*data).event_flag == ImGuiInputTextFlags::CallbackResize {
                while (*s).len() < (*data).buf_size as usize {
                    (*s).push('\0');
                }
                if (*s).len() > (*data).buf_size as usize {
                    (*s).truncate((*data).buf_size as usize);
                }
                (*data).buf = (*s).as_ptr() as *mut c_char;
            }
        }
        0
    }

    #[inline]
    pub fn input_text(&self, label: &CStr, s: &mut String, extra_flags: impl Into<Option<ImGuiInputTextFlags>>) {
        s.push('\0');
        unsafe {
            igInputText(
                label.as_ptr(),
                s.as_ptr() as *mut c_char,
                s.len(),
                match extra_flags.into() {
                    Some(v) => v | ImGuiInputTextFlags::CallbackResize,
                    None => ImGuiInputTextFlags::CallbackResize,
                },
                Some(Self::input_text_callback),
                s as *mut String as *mut c_void,
            );
        }
        s.pop();
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
        unsafe { ::std::ffi::CStr::from_ptr(cstr_ptr!($s)) }
    );
    ($s:expr, $($arg:expr),*) => (
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(&format!(concat!($s, "\0"), $($arg),*).into_bytes()) }
    );
}
