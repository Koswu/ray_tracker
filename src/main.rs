

mod shape;
mod ray;
mod scene;
mod renderer;
mod camera;
mod writer;
mod light;
mod colors;
mod types;

use camera::Camera;
use colors::PixelInPicutre;
use image::{ImageBuffer, Rgb};
use light::DotLight;
use nalgebra::{Vector3, Vector};
use renderer::{Renderer, PhongMaterial};
use scene::Scene;
use shape::{sphere::Sphere, Material};
use writer::ImageWriter;


fn build_scene() -> Scene{
    let mut scene = scene::Scene::new(Rgb([0.2, 0.2, 0.2]), 0.01);
    let material = Material::new(Rgb([0.8, 0.0, 0.0]),Rgb([1.0, 0.0, 0.0]), Rgb([1.0, 1.0, 1.0]), 32.0);
    let light = DotLight{ position: Vector3::<f64>::new(2.0, 2.0, 2.0), color_density: Rgb([1.0, 1.0, 1.0])};
    //let light2 = DotLight{ position: Vector3::<f64>::new(), color_density: todo!() }
    
    let sphere = Sphere::new(Vector3::<f64>::new(0.0, 0.0, 5.0), 1.0, material);
    scene.add_object(Box::new(sphere));
    scene.add_light(light);

    scene
}

fn get_camera() -> Camera{
    let camera = Camera::new(
        Vector3::<f64>::new(0.0, 0.0, 1.0),
        Vector3::<f64>::new(0.0, 1.0, 0.0),
        Vector3::<f64>::new(0.0, 0.0, 0.0),
        16.0/9.0,
        90.0,
    );
    camera
}

fn make_image(scene: Scene, camera: Camera) -> ImageBuffer<PixelInPicutre, Vec<u8>>{
    let writer =  ImageWriter::new(1920, 1080);
    let render = Renderer::new(scene, camera);
    let res = writer.write(&render);
    res
}

fn main() {
    let scene = build_scene();
    let camera = get_camera();

    let img = make_image(scene, camera);

    img.save("fin.png").unwrap();
    println!("generated");
}
