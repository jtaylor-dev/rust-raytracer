use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::{Hittable, HittableList, RotateY, Translate},
    material::*,
    math::{Color, Point3, Vec3},
    primitives::*,
    scene::*,
};
use std::sync::Arc;

pub fn cornell_smoke(camera: Camera, background: Color, use_bvh: bool) -> Scene {
    let mut scene_objects = HittableList::new();

    let mat_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
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
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        mat_light.clone(),
    ));

    // boxes
    let box0 = Arc::new(AaBox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        mat_white.clone(),
    ));
    let box0: Arc<dyn Hittable> = Arc::new(RotateY::new(box0.clone(), 15.0));
    let box0: Arc<dyn Hittable> =
        Arc::new(Translate::new(box0.clone(), Vec3::new(265.0, 0.0, 295.0)));
    scene_objects.add(ConstantMedium::from_color(
        box0,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    ));

    let box1 = Arc::new(AaBox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        mat_white.clone(),
    ));
    let box1: Arc<dyn Hittable> = Arc::new(RotateY::new(box1.clone(), -18.0));
    let box1: Arc<dyn Hittable> =
        Arc::new(Translate::new(box1.clone(), Vec3::new(130.0, 0.0, 65.0)));
    scene_objects.add(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    ));

    if use_bvh {
        let bvh = BvhNode::from_list(&scene_objects, 0.0, 1.0);
        println!("Created root BvhNode: {}", bvh);
        Scene::new(bvh.into(), background, camera)
    } else {
        Scene::new(scene_objects, background, camera)
    }
}
