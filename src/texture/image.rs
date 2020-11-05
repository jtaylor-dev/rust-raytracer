use super::Texture;
use crate::math::{Color, Point3};
use image::open;
use num::clamp;

const CHANNELS: usize = 3;
const SCALE: f64 = 1.0 / 255.0;

pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: usize,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        let img_rgb8 = open(filename).unwrap().as_rgb8().unwrap().to_owned();
        Self {
            bytes_per_scanline: CHANNELS * img_rgb8.width() as usize,
            width: img_rgb8.width(),
            height: img_rgb8.height(),
            data: img_rgb8.into_raw(),
        }
    }
}

impl Texture for Image {
    fn sample(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;

        if i >= self.width as i32 {
            i = self.width as i32 - 1;
        }

        if j >= self.height as i32 {
            j = self.height as i32 - 1;
        }

        let offset = j as usize * self.bytes_per_scanline + i as usize * CHANNELS;
        let pixel = self.data.get(offset..offset + 3).unwrap();

        SCALE * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}
