extern crate winapi;
extern crate user32;

use widgets;

pub struct Window {
    hwnd: winapi::HWND,
}

pub trait IntoWindow {
    fn into_window(&self) -> &Window;
}

trait HasLabel { }

fn to_wchar(text: &str) -> Vec<winapi::WCHAR> {
}

impl<T> widgets::Label for T
where T : IntoWindow + HasLabel {

    fn get_text(&self) -> String {
        let hwnd = self.into_window().hwnd;
        let size = unsafe { user32::GetWindowTextLengthW(hwnd) };
        let wtext = Vec::<winapi::WCHAR>::with_capacity(size);
        unsafe {
            let len = user32::GetWindowTextW(hwnd, wtext.as_mut_ptr(), wtext.capacity());
            wtext.set_size(len);
        }
    }

    fn set_text(&mut self, text: &str) {
        let hwnd = self.into_window().hwnd;
        let wtext = to_wchar(text);
        unsafe {
            user32::SetWindowTextW(hwnd, wtext.as_ptr());
        }
    }

}
