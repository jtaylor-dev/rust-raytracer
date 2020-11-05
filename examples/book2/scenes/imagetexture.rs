use raytracer::{
    bvh::BvhNode, hittable::Scene, material::*, math::Point3, primitives::Sphere, texture::*,
};
use std::sync::Arc;

pub fn earthmap(use_bvh: bool) -> Scene {
    let mut scene_objects = Scene::new();

    let earth: Arc<dyn Texture> = Arc::new(Image::new("assets/earthmap.jpg"));

    let mat_earth: Arc<dyn Material> = Arc::new(Lambertian::from(earth));

    scene_objects.add(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        mat_earth.clone(),
    ));

    if use_bvh {
        let bvh = BvhNode::from_list(&scene_objects, 0.0, 1.0);
        println!("Created root BvhNode: {}", bvh);
        let mut scene = Scene::new();
        scene.add(bvh);
        scene
    } else {
        scene_objects
    }
}
