// 用于存放全局可用的结构体，不包含逻辑
use nalgebra::{Vector3, Unit};

use crate::renderer::PhongMaterial;



#[derive(Debug, Clone)]
pub struct HitInfo<'a>{
    pub hit_distance: f64,
    pub hit_point: Vector3<f64>,
    pub hit_normal: Unit<Vector3<f64>>,
    pub material: Box<&'a dyn PhongMaterial>,
}
