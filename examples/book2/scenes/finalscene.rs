use rand::{thread_rng, Rng};
use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::{Hittable, HittableList, RotateY, Translate},
    material::*,
    math::{Color, Point3, Vec3},
    primitives::*,
    scene::*,
    texture::*,
};
use std::sync::Arc;

pub fn final_scene(camera: Camera, background: Color, _use_bvh: bool) -> Scene {
    let mut boxes_0 = HittableList::new();

    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = thread_rng().gen_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes_0.add(AaBox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                mat_ground.clone(),
            ));
        }
    }

    let mut scene_objects = HittableList::new();

    scene_objects.add(BvhNode::from_list(&boxes_0, 0.0, 1.0));

    let mat_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    scene_objects.add(XzPlane::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        mat_light.clone(),
    ));

    let center_0 = Point3::new(400.0, 400.0, 200.0);
    let center_1 = center_0 + Vec3::new(30.0, 0.0, 0.0);

    let mat_moving_sphere: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    scene_objects.add(MovingSphere::new(
        center_0,
        center_1,
        50.0,
        0.0,
        1.0,
        mat_moving_sphere.clone(),
    ));

    scene_objects.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene_objects.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    ));

    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    scene_objects.add(boundary);
    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    scene_objects.add(ConstantMedium::from_color(
        Arc::new(boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));

    let boundary = Sphere::new(Point3::default(), 5000.0, Arc::new(Dielectric::new(1.5)));
    scene_objects.add(ConstantMedium::from_color(
        Arc::new(boundary),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ));

    let earth: Arc<dyn Texture> = Arc::new(Image::new("assets/earthmap.jpg"));
    let mat_earth: Arc<dyn Material> = Arc::new(Lambertian::from(earth));
    scene_objects.add(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        mat_earth.clone(),
    ));

    let perlin: Arc<dyn Texture> = Arc::new(PerlinNoise::new(0.1));
    let mat_perlin: Arc<dyn Material> = Arc::new(Lambertian::from(perlin));
    scene_objects.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        mat_perlin.clone(),
    ));

    let mut boxes_1 = HittableList::new();
    let mat_white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes_1.add(Sphere::new(
            Point3::random_in_range(0.0, 165.0),
            10.0,
            mat_white.clone(),
        ));
    }

    let bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from_list(&boxes_1, 0.0, 1.0));
    let rotate = RotateY::new(bvh, 15.0);
    let translate = Translate::new(Arc::new(rotate), Vec3::new(-100.0, 270.0, 395.0));
    scene_objects.add(translate);

    let mut scene_list = HittableList::new();
    scene_list.add(BvhNode::from_list(&scene_objects, 0.0, 1.0));
    Scene::new(scene_list, background, camera)
}
