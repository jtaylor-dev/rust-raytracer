use rand::{thread_rng, Rng};
use raytracer::{
    bvh::BvhNode,
    hittable::Scene,
    material::*,
    math::{Color, Point3, Vec3},
    primitives::{MovingSphere, Sphere},
    texture::*,
};
use std::sync::Arc;

pub fn random_scene(use_bvh: bool) -> Scene {
    let mut scene_objects = Scene::new();

    let checker_texture: Arc<dyn Texture> = Arc::new(CheckerPattern::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::from(checker_texture));
    scene_objects.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground.clone(),
    ));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material>;
                if choose_mat < 0.6 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    material = Arc::new(Lambertian::new(albedo));
                    scene_objects.add(Sphere::new(center, 0.2, material));
                } else if choose_mat < 0.8 {
                    // moving diffuse sphere
                    let albedo = Vec3::random() * Vec3::random();
                    material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    scene_objects.add(MovingSphere::new(center, center2, 0.2, 0.0, 1.0, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz: f64 = rng.gen_range(0.0, 0.5);
                    material = Arc::new(Metal::new(albedo, fuzz));
                    scene_objects.add(Sphere::new(center, 0.2, material));
                } else {
                    // glass
                    material = Arc::new(Dielectric::new(1.5));
                    scene_objects.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    let glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let lambert: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    scene_objects.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.25, glass.clone()));
    scene_objects.add(Sphere::new(
        Point3::new(-4.0, 1.0, 3.0),
        1.0,
        lambert.clone(),
    ));
    scene_objects.add(Sphere::new(
        Point3::new(-4.0, 1.0, -3.0),
        1.0,
        lambert.clone(),
    ));
    scene_objects.add(Sphere::new(Point3::new(4.0, 1.0, -2.0), 1.0, metal.clone()));

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
