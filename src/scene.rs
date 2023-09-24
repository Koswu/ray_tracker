
pub mod stub;
use std::fmt::Debug;

use nalgebra::{Vector3, Unit};

use crate::colors::PixelInWorldTrait;
use crate::renderer::Illumination;
use crate::{ray::RayPath, renderer::PhongScene, light::DotLight, colors::PixelInWorld};
use crate::types::HitInfo;



pub trait GetVerticesable{
    fn get_vertices(&self, position: Vector3<f64>) -> Vec<Vector3<f64>> ; // 用于检查可达性
}


pub trait SceneShape{
    fn get_hit_info(&self, ray: &RayPath) -> Option<HitInfo>;
}


pub struct Scene {
    objects: Vec<Box<dyn SceneShape>>,
    lights: Vec<DotLight>,
    ambient_intensity: PixelInWorld,
    attenuation_coefficient: f64,
}


impl Scene {
    pub fn new(ambient_intensity: PixelInWorld, attenuation_coefficient: f64) -> Self{
        Scene { objects: vec![], lights: vec![], ambient_intensity, attenuation_coefficient }
    }
    pub fn add_object(& mut self, obj: Box<dyn SceneShape>){
        self.objects.push(obj)
    }

    pub fn add_light(& mut self, light: DotLight){
        self.lights.push(light)
    }
}

impl PhongScene for Scene{
    fn get_ambient_intensity(&self) -> PixelInWorld {
        self.ambient_intensity
    }

    fn get_hit_info(&self, ray: &RayPath) -> Option<HitInfo> {
        self.objects.iter()
        .filter_map(|obj| obj.get_hit_info(ray))
        .fold(None , |hit_info: Option<HitInfo>, another: HitInfo| {
            match hit_info{
                None => Some(another),
                Some(x) => Some(if x.hit_distance < another.hit_distance {x} else{another})
            }
        })    
    }

    fn get_light_illuminations(&self, target: Vector3<f64>) -> Vec<Illumination> {
        self.lights.iter().filter_map(|light| {
            let direction = Unit::new_normalize(target - light.position);
            let ray = RayPath::new(light.position, direction);
            let hit_info = self.get_hit_info(&ray).filter(|info| {
                (info.hit_point - target).norm() < 1e-7
            });
            hit_info.map(|info|{
                let attenuation = 1.0 / (1.0 + self.attenuation_coefficient * info.hit_distance * info.hit_distance);
                Illumination{
                    to_target_direction: direction,
                    intensity: light.color_density.scaling(attenuation),
                }
            })
        }).collect()
    }
}


