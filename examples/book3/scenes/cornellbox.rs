use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::*,
    material::*,
    math::{Color, Point3, Vec3},
    primitives::*,
    scene::*,
};
use std::sync::Arc;

pub fn cornell_box(camera: Camera, background: Color) -> Scene {
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
        Some(mat_green.clone()),
    ));
    scene_objects.add(YzPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Some(mat_red.clone()),
    ));

    scene_objects.add(XzPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Some(mat_white.clone()),
    ));
    scene_objects.add(XzPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Some(mat_white.clone()),
    ));

    scene_objects.add(XyPlane::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Some(mat_white.clone()),
    ));

    // top light
    let light_plane: Arc<dyn Hittable> = Arc::new(XzPlane::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Some(mat_light.clone()),
    ));
    let flipped = FlipFace::new(light_plane.clone());
    scene_objects.add(flipped);

    // boxes
    //let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let box0 = Arc::new(AaBox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Some(mat_white.clone()),
    ));
    let box0: Arc<dyn Hittable> = Arc::new(RotateY::new(box0.clone(), 15.0));
    let box0 = Translate::new(box0.clone(), Vec3::new(265.0, 0.0, 295.0));
    scene_objects.add(box0);

    // glass sphere
    let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    scene_objects.add(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        Some(mat_glass.clone()),
    ));

    let bvh = BvhNode::from_list(&scene_objects, 0.0, 1.0);
    println!("Created root BvhNode: {}", bvh);

    let mut lights = HittableList::new();

    lights.add(XzPlane::new(213.0, 343.0, 227.0, 332.0, 554.0, None));
    lights.add(Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, None));
    Scene::new(bvh.into(), Arc::new(lights), background, camera)
}
