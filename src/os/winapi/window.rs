extern crate winapi;
extern crate user32;

use os::winapi::Error;
use widgets::{Point, Size, Rect};

use self::winapi::{DWORD, c_int};
use std::ptr::{null, null_mut};

#[derive(Debug)]
pub struct Window {
    hwnd: winapi::HWND,
}

pub struct WindowType<'t> {
    pub class: &'t str,
    pub style: winapi::DWORD,
    pub stylex: winapi::DWORD,
}

pub trait IntoWindow {
    fn into_window(&self) -> &Window;
}

impl Window {
    pub fn create(window_type: &WindowType, title: &str, parent: Option<Window>, rect: &Rect) -> Result<Window, Error> {
        let raw_parent = parent.map_or(0 as winapi::HWND, |w| w.hwnd);
        let hwnd = unsafe { user32::CreateWindowExA(window_type.stylex,
                                                    window_type.class.as_ptr() as *const i8,
                                                    title.as_ptr() as *const i8,
                                                    window_type.style,
                                                    rect.p.x as c_int, rect.p.y as c_int,
                                                    rect.s.w as c_int, rect.s.h as c_int,
                                                    raw_parent,
                                                    0 as winapi::HMENU, 0 as winapi::HINSTANCE, null_mut())};
        // FIXME
        if ! hwnd.is_null() {
            unsafe {
                user32::ShowWindow(hwnd, 1);
                user32::UpdateWindow(hwnd);
            }
        }
        if hwnd.is_null() { Err(Error::get()) } else { Ok(Window { hwnd : hwnd }) }
    }
}
