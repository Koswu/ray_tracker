

mod shape;
mod ray;
mod scene;
mod renderer;
mod camera;
mod writer;

use camera::Camera;
use nalgebra::{Vector3, Vector2};
use renderer::Renderer;
use scene::Scene;
use writer::ImageWriter;



fn main() {
    let writer =  ImageWriter::new(800, 600);
    let scene = Scene::new();
    let camera = Camera::new(
        Vector3::<f64>::new(10.0, 100.0, 100.0),
        Vector3::<f64>::new(10.0, 100.0, 100.0),
        Vector3::<f64>::new(10.0, 100.0, 100.0),
        16.0/9.0,
        50.0,
    );
    let img = {
        //let render = Renderer::new(scene, camera);
        let render = renderer::stub::DummyRenderer::new();
        let res = writer.write(&render);
        res
    };
    img.save("fin.png").unwrap();
}
