
pub mod stub;
use image::Rgb;

use crate::{scene::Scene, camera::Camera, ray::{ReverseRay, ForwardRay}};

pub struct Renderer {
    scene: Scene,
    camera: Camera,
}


impl Renderer{
    pub fn new(scene: Scene, camera: Camera) -> Self{
        Renderer { scene: scene, camera: camera }
    }
    fn cal_ray(&self, ray: ReverseRay) -> ForwardRay{
        let nearest = self.scene.get_first_hit(&ray);

        let mut forward_rays = vec![];


        let color = match nearest {
            Some(nearest) => {
                let next_rays = nearest.material.get_secondary_reverse_rays(&ray, nearest.geometry_info);
                for next_ray in next_rays{
                    forward_rays.push(self.cal_ray(next_ray));
                }
                nearest.material.get_forward_ray_color(forward_rays, nearest.geometry_info)
            }
            None => {
                Rgb::<u8>([0, 0, 0])
            }
        };
        ray.to_forward(color)
    }

    pub fn get_color(&self, u: f64, v: f64) -> Rgb<u8>{
        //u , v in [0, 1]
        let ray = self.camera.generate_ray_in_viewport(u, v);
        self.cal_ray(ray).color
    }
}