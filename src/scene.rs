//! Types for storing scene objects and rendering them

use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    math::{Color, Ray, Vec3},
};
use indicatif::ProgressBar;
use num::clamp;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

/// Stores a HittableList and extra scene data (like background)
pub struct Scene {
    hittables: HittableList,
    background: Color,
    camera: Camera,
}

impl Scene {
    pub fn new(hittables: HittableList, background: Color, camera: Camera) -> Self {
        Self {
            hittables,
            background,
            camera,
        }
    }

    /// Render a [`Scene`](crate::hittable::Scene) to an [`Image`](crate::image::Image)
    pub fn render(
        &self,
        image_width: usize,
        image_height: usize,
        samples: u64,
        max_depth: u64,
    ) -> Vec<u8> {
        let progress = ProgressBar::new(image_height as u64 * image_width as u64);

        let channels: usize = 3;
        let mut image = vec![0u8; image_width * image_height * channels];

        image
            .par_chunks_exact_mut(image_width * channels)
            .rev()
            .enumerate()
            .for_each(|(j, row)| {
                row.par_chunks_exact_mut(channels)
                    .enumerate()
                    .for_each(|(i, pixel)| {
                        let mut rng = thread_rng();
                        let mut sample_acc = Vec3::default();
                        for _ in 0..samples {
                            let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                            let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                            let ray = self.camera.get_ray(u, v);
                            sample_acc += ray_color(ray, self, max_depth);
                        }
                        sample_acc /= samples as i32;

                        // gamma correct
                        let r = (255.0 * clamp(sample_acc[0].sqrt(), 0., 0.999)) as u8;
                        let g = (255.0 * clamp(sample_acc[1].sqrt(), 0., 0.999)) as u8;
                        let b = (255.0 * clamp(sample_acc[2].sqrt(), 0., 0.999)) as u8;

                        pixel[0] = r;
                        pixel[1] = g;
                        pixel[2] = b;
                        progress.inc(1);
                    });
            });

        progress.finish();
        image
    }
}

fn ray_color(ray: Ray, scene: &Scene, depth: u64) -> Vec3 {
    if depth <= 0 {
        return Vec3::default();
    }

    if let Some(hit_rec) = scene.hittables.hit(&ray, 0.001, f64::INFINITY) {
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Vec3 = Vec3::default();
        if let Some(ref material) = hit_rec.material {
            let emitted = material.emitted(hit_rec.u, hit_rec.v, &hit_rec.point);
            if !material.scatter(&ray, &hit_rec, &mut attenuation, &mut scattered) {
                // no scatter
                return emitted;
            }

            return emitted + attenuation * ray_color(scattered, scene, depth - 1);
        }

        // no material - use red to stand out
        Color::new(1.0, 0.0, 0.0)
    } else {
        // no hit
        scene.background
    }
}
