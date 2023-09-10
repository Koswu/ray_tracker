use image::Rgb;

use crate::writer::GetColorable;


pub struct DummyRenderer{

}
impl DummyRenderer{
    pub fn new() -> Self{
        DummyRenderer {}
    }
}
impl GetColorable for DummyRenderer{
    fn get_color(&self, u: f64, v: f64) -> Rgb<u8>{
        let r = (u * 255.0).round() as u8;
        let g = (v * 255.0).round() as u8;
        Rgb::<u8>([r, g, 0])
    }
}

