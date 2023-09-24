use image::ImageBuffer;

use crate::colors::{PixelInWorld, PixelInPicutre, PixelInWorldTrait};




pub trait GetColorable {
    fn get_color(&self, u: f64, v: f64) -> PixelInWorld;
}




pub struct ImageWriter{
    width: u32,
    height: u32,
}

impl ImageWriter{
    pub fn new(width: u32, height: u32) -> Self{
        ImageWriter { width: width, height: height}
    }
    pub fn write<T: GetColorable>(&self, renderer: &T) -> ImageBuffer<PixelInPicutre, Vec<u8>>{
        let mut buff = ImageBuffer::<PixelInPicutre, Vec<u8>>::new(self.width, self.height);
        for (x, y, color) in buff.enumerate_pixels_mut(){
            let u = x as f64 / self.width as f64;
            let v = 1.0 - (y as f64 / self.height as f64);
            *color = renderer.get_color(u, v).to_picture_pixel();
        }
        buff
    }
}
