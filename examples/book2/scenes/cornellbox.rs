use raytracer::{
    bvh::BvhNode, camera::Camera, hittable::HittableList, material::*, math::Color, primitives::*,
    scene::*,
};
use std::sync::Arc;

pub fn cornell_box(camera: Camera, background: Color, use_bvh: bool) -> Scene {
    let mut scene_objects = HittableList::new();

    let mat_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    let mat_red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let mat_white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let mat_green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    // box walls
    scene_objects.add(YzPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        mat_green.clone(),
    ));
    scene_objects.add(YzPlane::new(0.0, 555.0, 0.0, 555.0, 0.0, mat_red.clone()));

    scene_objects.add(XzPlane::new(0.0, 555.0, 0.0, 555.0, 0.0, mat_white.clone()));
    scene_objects.add(XzPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        mat_white.clone(),
    ));

    scene_objects.add(XyPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        mat_white.clone(),
    ));

    // top light
    scene_objects.add(XzPlane::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        mat_light.clone(),
    ));

    if use_bvh {
        let bvh = BvhNode::from_list(&scene_objects, 0.0, 1.0);
        println!("Created root BvhNode: {}", bvh);
        Scene::new(bvh.into(), background, camera)
    } else {
        Scene::new(scene_objects, background, camera)
    }
}
