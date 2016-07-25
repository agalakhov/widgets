extern crate winapi;
extern crate user32;

mod window;
mod button;

#[derive(Debug)]
pub struct Error;

pub fn run_once() -> Result<bool, Error> {
  let mut msg = winapi::MSG {
      message : 0,
      pt : winapi::POINT { x : 0, y : 0 },
      hwnd : 0 as winapi::HWND,
      time : 0,
      lParam : 0,
      wParam : 0,
  };

  let result = unsafe { user32::GetMessageW(&mut msg, 0 as winapi::HWND, 0, 0) };
  if result < 0 {
      return Err(Error);
  }
  if result == 0 {
      return Ok(false);
  }

  unsafe { user32::TranslateMessage(&msg) };
  unsafe { user32::DispatchMessageW(&msg) };

  return Ok(true);
}
