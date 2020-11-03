use clap::{App, Arg, ArgMatches};
use image;
use rand::{thread_rng, Rng};
use raytracer::{
    bvh::BvhNode,
    camera::Camera,
    hittable::Scene,
    material::*,
    math::{Color, Point3, Vec3},
    primitives::{MovingSphere, Sphere},
    render::render_scene,
    texture::*,
};
use std::sync::Arc;

fn main() {
    let matches = match_args();

    // Parse camera, image args
    let image_height: usize = matches
        .value_of("height")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let image_width: usize = matches.value_of("width").unwrap().parse().unwrap();
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel: u64 = matches.value_of("samples").unwrap().parse().unwrap();
    let max_depth: u64 = matches.value_of("bounces").unwrap().parse().unwrap();
    let fov: f64 = matches.value_of("fov").unwrap().parse().unwrap();
    let aperture: f64 = matches.value_of("aperture").unwrap().parse().unwrap();
    let focus_distance: f64 = matches.value_of("focusdist").unwrap().parse().unwrap();
    let use_bvh = matches.is_present("bvh");

    // Parse filename
    let filename = matches.value_of("output").unwrap();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        fov,
        aspect_ratio,
        aperture,
        focus_distance,
        0.0,
        1.0,
    );

    println!(
        "Rendering scene to {}x{} image ({} pixels) with {} bounces/ray and {} samples/pixel",
        image_width,
        image_height,
        image_width * image_height,
        max_depth,
        samples_per_pixel,
    );
    if use_bvh {
        println!("BVH optimization = ON");
    }

    // Init scene
    let scene = random_scene(use_bvh);

    // Render
    let image = render_scene(
        &scene,
        &camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    );
    image::save_buffer(
        filename,
        &image,
        image_width as u32,
        image_height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

fn random_scene(use_bvh: bool) -> Scene {
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

fn match_args() -> ArgMatches<'static> {
    App::new("Raytracer")
        .version("0.1.0")
        .about("Renders a raytraced scene")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .takes_value(true)
                .required(true)
                .help("Sets image width of output"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .takes_value(true)
                .required(true)
                .help("Sets image height of output"),
        )
        .arg(
            Arg::with_name("fov")
                .short("f")
                .long("fov")
                .value_name("FOV")
                .takes_value(true)
                .default_value("20.0")
                .help("Sets field of vision (fov)"),
        )
        .arg(
            Arg::with_name("aperture")
                .short("a")
                .long("aperture")
                .value_name("APERTURE")
                .takes_value(true)
                .default_value("0")
                .help("Sets diameter of camera aperture (controls amount of defocus blur)"),
        )
        .arg(
            Arg::with_name("focusdist")
                .short("d")
                .long("focus-dist")
                .value_name("FOCUS")
                .takes_value(true)
                .default_value("10.0")
                .help("Sets distance to the focus plane (controls distance of defocus blur)"),
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .value_name("SAMPLES")
                .takes_value(true)
                .default_value("500")
                .help("Sets number of samples per pixel"),
        )
        .arg(
            Arg::with_name("bounces")
                .short("b")
                .long("bounces")
                .value_name("BOUNCES")
                .takes_value(true)
                .default_value("50")
                .help("Sets max bounces (depth) for each raycast"),
        )
        .arg(
            Arg::with_name("bvh")
                .long("bvh")
                .help("Use bounding volume hierarchy (BVH) optimizations"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .default_value("render.ppm")
                .help("File to save rendered image"),
        )
        .get_matches()
}
