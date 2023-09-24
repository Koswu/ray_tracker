use std::{cmp::min, ops::Deref};

use nalgebra::{Vector3, Unit};

use crate::{scene::SceneShape, types::HitInfo};

use super::Material;



pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self{
        Sphere{
            center,
            radius,
            material,
        }
    }
}

impl SceneShape for Sphere{
    fn get_hit_info(&self, ray: &crate::ray::RayPath) -> Option<crate::types::HitInfo> {
        // 列出光线与球体的方程，联立得到方程组
        // 设有一点 P ，同时在光线和球面上
        // O + kD = P   （光线方程, O 为原点，D 为方向）
        let o: nalgebra::Matrix<f64, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f64, 3, 1>> = ray.origin;
        let d = ray.direction;
        // (P - Q) * (P - Q) = r^2   (球体方程， Q 为球心，r 为半径)
        let q = self.center;
        let r = self.radius;
        // 联立求解方程组，可以 得到一个 ak^2 + bk + c = 0 的二次方程
        // 其中 a = D * D, b = 2D(O-Q), c = (O-Q) * (O-Q) - r^2
        let a = d.dot(&d);
        let t = o-q;
        let b = 2.0*d.dot(&t);
        let c = (t).dot(&t) - r*r;

        let delta = b*b-4.0*a*c;
        if delta < 0.0 {
            return None
        }
        let k = (-b + delta.sqrt()) / (2.0 * a).min(
            (-b - delta.sqrt()) / (2.0 * a),
        );
        let hit_point = o + (k * d.deref());
        //println!("hp = {:?}, k = {:?}", hit_point, k);
        Some(HitInfo{
            hit_distance: k,
            hit_point: hit_point,
            hit_normal: Unit::new_normalize(hit_point - self.center),
            material: Box::new(&self.material),
        })
    }
}


