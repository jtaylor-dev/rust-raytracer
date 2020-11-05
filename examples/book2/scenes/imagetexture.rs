use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::HittableList,
    material::*,
    math::{Color, Point3},
    primitives::Sphere,
    scene::Scene,
    texture::*,
};
use std::sync::Arc;

pub fn earthmap(camera: Camera, use_bvh: bool) -> Scene {
    let mut scene_objects = HittableList::new();

    let earth: Arc<dyn Texture> = Arc::new(Image::new("assets/earthmap.jpg"));

    let mat_earth: Arc<dyn Material> = Arc::new(Lambertian::from(earth));

    scene_objects.add(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        mat_earth.clone(),
    ));

    let background = Color::new(0.7, 0.8, 1.0);
    if use_bvh {
        let bvh = BvhNode::from_list(&scene_objects, 0.0, 1.0);
        println!("Created root BvhNode: {}", bvh);
        Scene::new(bvh.into(), background, camera)
    } else {
        Scene::new(scene_objects, background, camera)
    }
}
