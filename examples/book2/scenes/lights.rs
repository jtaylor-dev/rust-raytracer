use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::HittableList,
    material::*,
    math::{Color, Point3},
    primitives::*,
    scene::*,
    texture::*,
};
use std::sync::Arc;

pub fn lights_rgb(camera: Camera, background: Color, use_bvh: bool) -> Scene {
    let mut scene_objects = HittableList::new();

    let perlin: Arc<dyn Texture> = Arc::new(PerlinNoise::new(4.0));
    let mat_perlin: Arc<dyn Material> = Arc::new(Lambertian::from(perlin));

    scene_objects.add(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        mat_perlin.clone(),
    ));

    let mat_red_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(1.0, 0.0, 0.0)));
    scene_objects.add(XyPlane::new(
        -1.5,
        1.5,
        -1.5,
        1.5,
        -6.5,
        mat_red_light.clone(),
    ));

    let mat_blue_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(0.0, 0.0, 1.0)));
    scene_objects.add(XyPlane::new(
        -1.5,
        1.5,
        -1.5,
        1.5,
        6.5,
        mat_blue_light.clone(),
    ));

    let mat_green_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(0.0, 1.0, 0.0)));
    scene_objects.add(XzPlane::new(
        -2.5,
        1.5,
        -1.5,
        1.5,
        -6.5,
        mat_green_light.clone(),
    ));

    let mat_white_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(1.0, 1.0, 1.0)));
    scene_objects.add(XzPlane::new(
        -2.5,
        1.5,
        -1.5,
        1.5,
        6.5,
        mat_white_light.clone(),
    ));

    let mat_bg: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene_objects.add(YzPlane::new(
        -100.0,
        100.0,
        -100.0,
        100.0,
        -3.0,
        mat_bg.clone(),
    ));

    if use_bvh {
        let bvh = BvhNode::from_list(&scene_objects, 0.0, 1.0);
        println!("Created root BvhNode: {}", bvh);
        Scene::new(bvh.into(), background, camera)
    } else {
        Scene::new(scene_objects, background, camera)
    }
}
