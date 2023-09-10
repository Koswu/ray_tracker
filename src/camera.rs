use nalgebra::{Vector2, Vector3, Unit};

use crate::ray::ReverseRay;

pub struct Camera {
    look_at: Unit<Vector3<f64>>,
    up: Unit<Vector3<f64>>,
    position: Vector3<f64>,
    aspect_ratio: f64, // 横纵比
    h_fov: f64 // 水平场视角
}

impl Camera{
    pub fn new(look_at: Vector3<f64>, up: Vector3<f64>, position: Vector3<f64>, aspect_ratio: f64, h_fov_deg: f64) -> Self{
        assert!(look_at.dot(&up).abs() < 1e-6); // look_at 和 up 应该正交

        let look_at = Unit::new_normalize(look_at);
        let up = Unit::new_normalize(up);
        Camera { look_at, up, position: position, aspect_ratio: aspect_ratio, h_fov: h_fov_deg.to_radians() }
    }
    fn get_viewport_size(&self) -> Vector2<f64>{
        // 「底大一级压死人」
        // 视口大小类似相机的 CMOS 感光元件，由于这里是纯计算，底大不大都一样
        // 简单设为 1
        Vector2::<f64>::new(1.0, 1.0/self.aspect_ratio)
    }
    fn get_right(&self) -> Unit<Vector3<f64>>{
        Unit::new_normalize(self.look_at.cross(&self.up))
    }
    fn get_viewport_center(&self) -> Vector3<f64>{
        self.position + self.look_at.as_ref() * self.get_focal_length() 
    }
    fn get_focal_length(&self) -> f64{
        // 焦距计算
        // 焦距就是从相机位置到视口的距离
        // BTW, 相机的「位置」其实是一个虚拟的概念，表示反向光线发出的起点
        let viewpoint = self.get_viewport_size();
        let (u, _) = (viewpoint.x, viewpoint.y);
        u/(self.h_fov/2.0).tan()
    }
    pub fn generate_ray_in_viewport(&self, u: f64, v: f64) -> ReverseRay{
        // 获取经过视口的光线
        // u, v 分别为在两个方向上视口的位置，在 0-1 之间
        assert!(u >= 0.0 && u <= 1.0);
        assert!(v >= 0.0 && v <= 1.0);
        let u = u - 0.5;
        let v = v - 0.5;
        let viewport_center = self.get_viewport_center();
        let viewport_size = self.get_viewport_size();

        let vp_up_vec = self.up.as_ref() * viewport_size.y;
        let vp_right_vec = self.get_right().as_ref() * viewport_size.x;

        let vec_in_viewport = viewport_center + u * vp_right_vec + v * vp_up_vec;
        ReverseRay::new(self.position, vec_in_viewport, 10)
    }
}
