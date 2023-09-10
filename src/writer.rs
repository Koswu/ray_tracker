use image::{ImageBuffer, Rgb};

use crate::renderer::Renderer;

pub trait GetColorable {
    fn get_color(&self, u: f64, v: f64) -> Rgb<u8>;
}



pub struct ImageWriter{
    width: u32,
    height: u32,
}

impl ImageWriter{
    pub fn new(width: u32, height: u32) -> Self{
        ImageWriter { width: width, height: height}
    }
    pub fn write<T: GetColorable>(&self, renderer: &T) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
        let mut buff = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.width, self.height);
        for (x, y, color) in buff.enumerate_pixels_mut(){
            let u = x as f64 / self.width as f64;
            let v = y as f64 / self.height as f64;
            *color = renderer.get_color(u, v);
        }
        buff
    }
}
