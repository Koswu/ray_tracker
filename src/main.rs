

mod shape;
mod ray;
mod scene;
mod renderer;
mod camera;
mod writer;

use camera::Camera;
use image::{ImageBuffer, Rgb};
use nalgebra::{Vector3, Vector2};
use renderer::Renderer;
use scene::Scene;
use shape::sphere::Sphere;
use writer::ImageWriter;


fn build_scene() -> Scene{
    let mut scene = scene::Scene::new();
    
    let sphere = Sphere::new(Vector3::<f64>::new(0.0, 0.0, 100.0), 1.0);
    scene.add_object(Box::new(sphere));

    scene
}

fn get_camera() -> Camera{
    let camera = Camera::new(
        Vector3::<f64>::new(0.0, 0.0, 1.0),
        Vector3::<f64>::new(1.0, 0.0, 0.0),
        Vector3::<f64>::new(0.0, 0.0, 0.0),
        16.0/9.0,
        50.0,
    );
    camera
}

fn make_image(scene: Scene, camera: Camera) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
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
}
