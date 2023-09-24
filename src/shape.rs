use crate::colors::PixelInWorld;


pub mod sphere;
mod material;



#[derive(Debug)]
pub struct Material{
    ambient_coefficient: PixelInWorld,
    diffuse_coefficient: PixelInWorld,
    specular_coefficient: PixelInWorld,
    shininess: f64,
}
impl Material{
    pub fn new(ambient: PixelInWorld, diffuse: PixelInWorld, specular: PixelInWorld, shininess: f64) -> Self{
        Material{
            ambient_coefficient: ambient,
            diffuse_coefficient: diffuse,
            specular_coefficient: specular,
            shininess: shininess,
        }
    }
}
