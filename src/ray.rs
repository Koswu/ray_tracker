
use nalgebra::{Vector3, Unit};



#[derive(Debug, Clone, Copy)]
pub struct RayPath{
    pub origin: Vector3<f64>,
    pub direction: Unit<Vector3<f64>>,
}



impl RayPath {
    pub fn new(origin: Vector3<f64>, direction: Unit<Vector3<f64>>) -> Self{
        RayPath{ origin: origin, direction: direction}
    }
}
