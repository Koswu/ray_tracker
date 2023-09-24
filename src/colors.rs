

use image::{Rgb, Pixel};
use num_traits::{Float};

pub type PixelComponentInPicture = u8;
pub type PixelInPicutre = Rgb<PixelComponentInPicture>;

pub trait PixelInWorldTrait{
    const BLACK: Self;
    const WHITE: Self;
    fn mix(&self, other: &Self) -> Self;
    fn mask(&self, other: &Self) -> Self;
    fn scaling(&self, ratio: f64) -> Self;
    fn to_picture_pixel(&self) -> PixelInPicutre;
}


pub type PixelComponent = f64;

pub trait HasMinMaxValue {
    const MIN: Self;
    const MAX: Self;
}

impl HasMinMaxValue for PixelComponent{
    const MAX: Self = 1.0;
    const MIN: Self = 0.0;
}

pub trait SaturatingAdd<T = Self> {
    fn saturating_add(&self, v: &Self) -> Self;
}
pub trait SaturatingMul<T = Self> {
    fn saturating_mul(&self, v: &Self) -> T;
    
}


impl SaturatingAdd for PixelComponent{
    fn saturating_add(&self, v: &Self) -> Self {
        (self + v).clamp(Self::MIN, Self::MAX)
    }
}
impl SaturatingMul for PixelComponent{
    fn saturating_mul(&self, v: &Self) -> Self {
        (self * v).clamp(Self::MIN, Self::MAX)
    }
}

pub type PixelInWorld = Rgb<PixelComponent>;


impl PixelInWorldTrait for PixelInWorld{
    const BLACK: Self = Self([0.0, 0.0, 0.0]);
    const WHITE: Self = Self([1.0, 1.0, 1.0]);
    fn mix(&self, other: &Self) -> Self {
        Self([
            self[0].saturating_add(&other[0]),
            self[1].saturating_add(&other[1]),
            self[2].saturating_add(&other[2]),
        ])
    }
    fn mask(&self, other: &Self) -> Self {
        Self([
            self[0].saturating_mul(&other[0]),
            self[1].saturating_mul(&other[1]),
            self[2].saturating_mul(&other[2]),
        ])
    }
    fn scaling(&self, ratio: f64) -> Self {
        Self([
            self[0].saturating_mul(&ratio),
            self[1].saturating_mul(&ratio),
            self[2].saturating_mul(&ratio),
        ])
    }
    fn to_picture_pixel(&self) -> PixelInPicutre {
        Rgb::<u8>([
            (self[0] * 255.0) as u8,
            (self[1] * 255.0) as u8, 
            (self[2] * 255.0) as u8,
        ])
    }
}



