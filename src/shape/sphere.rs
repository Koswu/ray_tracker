use nalgebra::Vector3;

use crate::{scene::CheckIsHitAble, ray::ReverseRay};


pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Self{
        Sphere{
            center: center,
            radius: radius,
        }
    }
}

impl CheckIsHitAble for Sphere {
    fn check_is_hit(&self, ray: &ReverseRay) -> bool {
        // 列出光线与球体的方程，联立得到方程组
        // 设有一点 P ，同时在光线和球面上
        // O + kD = P   （光线方程, O 为原点，D 为方向）
        let o: nalgebra::Matrix<f64, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f64, 3, 1>> = ray.get_path().origin;
        let d = ray.get_path().direction;
        // (P - Q) * (P - Q) = r^2   (球体方程， Q 为球心，r 为半径)
        let q = self.center;
        let r = self.radius;
        // 联立求解方程组，可以 得到一个 ak^2 + bk + c = 0 的二次方程
        // 其中 a = D, b = 2D(O-Q), c = (O-Q) * (O-Q) - r^2
        let a = d.dot(&d);
        let t = o-q;
        let b = 2.0*d.dot(&t);
        let c = (t).dot(&t) - r*r;

        // 根据 delta 判别式 可知方程组是否有解
        let delta = b*b-4.0*a*c;
        delta >= 0.0
        
    }

}

