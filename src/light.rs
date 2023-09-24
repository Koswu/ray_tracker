use nalgebra::Vector3;

use crate::colors::PixelInWorld;



pub struct DotLight{
    pub position: Vector3<f64>,
    pub color_density: PixelInWorld,
}



