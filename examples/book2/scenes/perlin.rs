use raytracer::{
    bvh::BvhNode, hittable::Scene, material::*, math::Point3, primitives::Sphere, texture::*,
};
use std::sync::Arc;

pub fn two_perlin_spheres(use_bvh: bool) -> Scene {
    let mut scene_objects = Scene::new();

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
