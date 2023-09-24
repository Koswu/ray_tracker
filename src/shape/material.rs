

use std::ops::Deref;

use nalgebra::{Vector3, Norm, Unit};

use crate::{renderer::PhongMaterial, colors::{PixelInWorld, PixelInWorldTrait}};

use super::Material;



impl PhongMaterial for Material{
    fn get_ambient_reflection_intensity(&self, ambient_intensity: PixelInWorld) -> PixelInWorld {
        ambient_intensity.mask(&self.ambient_coefficient)
    }
    fn get_diffuse_intensity(&self, source_intensity: PixelInWorld, normal: Unit<Vector3<f64>>, to_light_direction: Unit<Vector3<f64>>) -> PixelInWorld {
        //println!("source = {:?}, normal {:?}, to_light_dir {:?}", source_intensity, normal, to_light_direction);
        //TODO: why negative work?
        let scaler = -normal.dot(to_light_direction.deref());
        //println!("scalar = {:?}", scaler);
        source_intensity.mask(&self.diffuse_coefficient).scaling(scaler)
    }
    fn get_specular_intensity(&self, souce_intensity: PixelInWorld, to_observer_direction: Unit<Vector3<f64>>, reflection_direction: Unit<Vector3<f64>>) -> PixelInWorld {
        let ratio = (to_observer_direction.deref().dot(reflection_direction.deref())).powf(self.shininess);
        //println!("ratio = {:?}, source_intensity = {:?}", ratio, souce_intensity);
        souce_intensity.mask(&self.specular_coefficient).scaling(
            ratio
        )
    }
}
