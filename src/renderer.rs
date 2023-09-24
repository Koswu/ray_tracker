
pub mod stub;

use std::{fmt::Debug, ops::Deref};

use nalgebra::{Vector3, Unit};

use crate::{ray::RayPath, camera::Camera, writer::GetColorable, colors::{PixelInWorld, PixelInWorldTrait}, types::HitInfo};




pub trait PhongMaterial: Debug{
    fn get_ambient_reflection_intensity(&self, ambient_intensity: PixelInWorld) -> PixelInWorld;

    fn get_diffuse_intensity(&self, source_intensity: PixelInWorld, normal: Unit<Vector3<f64>>, to_light_direction: Unit<Vector3<f64>>) -> PixelInWorld;

    fn get_specular_intensity(&self, souce_intensity: PixelInWorld, to_observer_direction: Unit<Vector3<f64>>, reflection_direction: Unit<Vector3<f64>>) -> PixelInWorld;
}



#[derive(Debug)]
pub struct Illumination{
    pub to_target_direction: Unit<Vector3<f64>>,
    pub intensity: PixelInWorld,
}



pub trait PhongScene{ 

    fn get_hit_info(&self, ray: &RayPath) -> Option<HitInfo>;

    fn get_light_illuminations(&self, target: Vector3<f64>) -> Vec<Illumination>;

    fn get_ambient_intensity(&self) -> PixelInWorld;
}


pub struct Renderer<T: PhongScene>{
    scene: T,
    camera: Camera,
}


impl <T: PhongScene>Renderer<T>{
    pub fn new(scene: T, camera: Camera) -> Self{
        Renderer { scene, camera }
    }
    fn get_ambient_intensity(&self, hit_info: &HitInfo) -> PixelInWorld {
        hit_info.material.get_ambient_reflection_intensity(self.scene.get_ambient_intensity())
    }
    fn get_diffuse_intensity(&self, illuminations: &[Illumination], hit_info: &HitInfo) -> PixelInWorld{
        illuminations.iter().fold(
            PixelInWorld::BLACK, |color, illumination| 
            color.mix(
            &hit_info.material.get_diffuse_intensity(
                illumination.intensity, 
                hit_info.hit_normal, 
                -illumination.to_target_direction)
            )
        )

    }
    fn get_specular_intensity(&self, illuminations: &[Illumination], hit_info: &HitInfo, ray: &RayPath) -> PixelInWorld{
        illuminations.iter().fold(
            PixelInWorld::BLACK, |color, illumination| {
                let n = hit_info.hit_normal.deref();
                let l = -illumination.to_target_direction.deref();
                let reflection_direction = 2.0*(n.dot(&l))*n - l;
                let res = hit_info.material.get_specular_intensity(
                        illumination.intensity, 
                        -ray.direction, 
                        Unit::new_normalize(reflection_direction),
                );
                //println!("res = {:?}", res);
                color.mix(
                    &res
                )
            }
        )

    }

    pub fn get_intensity(&self, ray: &RayPath) -> PixelInWorld{
        // 使用 Phong 光照模型，将光对物体的影响分为三类：环境光、漫反射光、高光
        let black_color = PixelInWorld::BLACK;
        let hit_info = self.scene.get_hit_info(ray);
        let hit_info = match hit_info {
            Some(value) => value,
            None => return black_color,
        };
        let illuminations : Vec<Illumination> = self.scene.get_light_illuminations(hit_info.hit_point);

        let i_ambient = self.get_ambient_intensity(&hit_info);
        let i_diffuse = self.get_diffuse_intensity(&illuminations, &hit_info);
        let i_specular = self.get_specular_intensity(&illuminations, &hit_info, ray);

        i_ambient.mix(&i_diffuse).mix(&i_specular)
    }

}

impl <T:PhongScene>GetColorable for Renderer<T> {
    fn get_color(&self, u: f64, v: f64) -> PixelInWorld {
        self.get_intensity(&self.camera.generate_ray_in_viewport(u, v))
    }
}
