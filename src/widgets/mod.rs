pub struct Point {
    pub x : u32,
    pub y : u32,
}

pub struct Size {
    pub w : u32,
    pub h : u32,
}

pub struct Rect {
    pub p : Point,
    pub s : Size,
}

pub trait Label {
    fn get_text(&self) -> String;
    fn set_text(&mut self, text: &str);
}

pub trait Button {
    fn set_on_clicked<F:Fn()>(func: F);
}
