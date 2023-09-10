use core::time;

use image::Rgb;
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct RayPath{
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl RayPath {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self{
        RayPath{ origin: origin, direction: direction }
    }
}


// 逆向光线，只表示路径信息
pub struct ReverseRay {
    path: RayPath,
    time_to_live: i64,
}

pub struct ForwardRay{
    pub path: RayPath,
    pub color: Rgb<u8>,
}

pub trait HitCheckable {
    fn check_is_hit(&self, ray: &ReverseRay) -> bool;
}

impl ForwardRay{
    pub fn new(path: RayPath, color: Rgb<u8>) -> Self{
        ForwardRay { path: path, color: color }
    }
}

impl ReverseRay {

    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, time_to_live: i64) -> Self{
        ReverseRay { 
            path: RayPath::new(origin, direction),
            time_to_live: time_to_live,
        }
    }

    pub fn create_next(&self, origin: Vector3<f64>, direction: Vector3<f64>) -> Option<Self>{
        if self.time_to_live <= 0{
            return None
        }
        Some(ReverseRay { path: RayPath { origin: origin, direction: direction }, time_to_live: self.time_to_live - 1 })

    }
    pub fn get_path(&self) -> &RayPath{
        &self.path
    }
    pub fn to_forward(&self, color: Rgb<u8>) -> ForwardRay{
        ForwardRay::new(self.path, color)
    }
}

