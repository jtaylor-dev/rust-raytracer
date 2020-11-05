use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::HittableList,
    material::*,
    math::{Color, Point3},
    primitives::Sphere,
    scene::*,
    texture::*,
};
use std::sync::Arc;

pub fn two_perlin_spheres(camera: Camera, use_bvh: bool) -> Scene {
    let mut scene_objects = HittableList::new();

    let perlin: Arc<dyn Texture> = Arc::new(PerlinNoise::new(4.0));

    let mat_perlin: Arc<dyn Material> = Arc::new(Lambertian::from(perlin));

    scene_objects.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_perlin.clone(),
    ));

    scene_objects.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        mat_perlin.clone(),
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
