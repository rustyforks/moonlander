use cairo::Context;
use pango::Layout;
use std::ops::Deref;

pub trait Line<C: Deref<Target = Context>> {
    fn get_size(&self) -> (f64, f64);
    fn draw(&mut self, ctx: &C, pango: &Layout);

    fn click(&mut self, _data: &super::Data) -> Option<super::Msg> {
        None // implementation optional
    }
}
