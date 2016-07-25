
pub trait Label {
    fn get_text(&self) -> String;
    fn set_text(&mut self, text: &str);
}

pub trait Button {
    fn set_on_clicked<F:Fn()>(func: F);
}
