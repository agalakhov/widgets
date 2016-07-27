extern crate gui;

use gui::widgets::*;
use gui::window::*;

extern crate winapi;

fn main() {
   let wt = WindowType {
      class: "Edit",
      style: winapi::WS_OVERLAPPEDWINDOW,
      stylex: 0,
   };

   let wr = Rect {
      p: Point {
         x: winapi::CW_USEDEFAULT as u32,
         y: winapi::CW_USEDEFAULT as u32,
      },
      s: Size {
         w: winapi::CW_USEDEFAULT as u32,
         h: winapi::CW_USEDEFAULT as u32,
      },
   };

   let win = Window::create(&wt, "Hello", None, &wr);
   println!("{:?}", win);

   gui::run().unwrap();
}
