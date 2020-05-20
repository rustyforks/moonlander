use cairo::Context;
use pango::Layout;

pub trait Line {
    fn get_pos(&self) -> (f64, f64);
    fn get_size(&self) -> (f64, f64);

    fn draw(&mut self, ctx: &Context, pango: &Layout, theme: &super::Theme);

    fn get_tooltip(&self, _data: &super::Data) -> Option<String> {
        None // implementation optional
    }

    fn click(&mut self, _data: &super::Data) -> Option<super::Msg> {
        None // implementation optional
    }
}
