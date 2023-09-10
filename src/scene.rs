use image::Rgb;
use nalgebra::Vector3;

use crate::ray::{ReverseRay, ForwardRay};

type Color = Rgb<u8>;

#[derive(Debug, Clone, Copy)]
pub struct HitGeometryInfo {
    pub position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub to_ray_source_distance: f64,
}

pub trait Material {
    fn get_secondary_reverse_rays(&self, incident_ray: &ReverseRay, hit_geometry_info: HitGeometryInfo) -> Vec<ReverseRay>;

    fn get_forward_ray_color(&self, incidnet_rays: Vec<ForwardRay>, hit_geometry_info: HitGeometryInfo) -> Color;

}

pub struct HitInfo {
    pub geometry_info: HitGeometryInfo,
    pub material: dyn Material,
}

pub trait CalHitable {
    fn cal_hit(&self, incident_ray: &ReverseRay) -> Option<Box<HitInfo>>;
}

pub struct DotLightSource{
    pub position: Vector3<f64>,
    pub color: Rgb<u8>,
}

pub struct Scene {
    objects: Vec<Box<dyn CalHitable>>,
    dot_lights: Vec<DotLightSource>,
}

impl Scene {
    pub fn new() -> Self{
        Scene { objects: vec![], dot_lights: vec![]}
    }
    pub fn add_object(& mut self, obj: Box<dyn CalHitable>){
        self.objects.push(obj)
    }
    pub fn get_objects(& self) -> &Vec<Box<dyn CalHitable>>{
        &self.objects
    }
    pub fn add_dot_light(& mut self, light: DotLightSource){
        self.dot_lights.push(light)
    }
    pub fn get_dot_lights(& self) -> &Vec<DotLightSource>{
        &self.dot_lights
    }
    pub fn get_first_hit(&self, ray: &ReverseRay) -> Option<Box<HitInfo>>{
        let mut hitinfos = vec![];
        for obj in &self.objects{
            let hitinfo = obj.cal_hit(&ray);
            if let Some(hit) = hitinfo {
                hitinfos.push(hit)
            }
        }

        let nearest = hitinfos.into_iter().filter(
            |hit| 
            hit.geometry_info.to_ray_source_distance.is_normal()
        ).min_by(
            |hit_a, hit_b| 
            hit_a.geometry_info.to_ray_source_distance.partial_cmp(
                &hit_b.geometry_info.to_ray_source_distance
            ).unwrap() 
        );
        nearest
    }
}





