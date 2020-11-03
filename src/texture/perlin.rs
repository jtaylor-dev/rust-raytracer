use super::Texture;
use crate::math::{Color, Point3, Vec3};
use rand::{thread_rng, Rng};

const POINT_COUNT: usize = 256;
const N: i32 = POINT_COUNT as i32 - 1;

pub struct PerlinNoise {
    perlin: Perlin,
    scale: f64,
}

impl PerlinNoise {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for PerlinNoise {
    #[allow(unused_variables)]
    fn sample(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.perlin.turb(p, 7)).sin())
    }
}

struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = [Vec3::default(); POINT_COUNT];
        ranvec
            .iter_mut()
            .for_each(|f| *f = Vec3::random_in_range(-1.0, 1.0).unit());
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di) & N) as usize]
                        ^ self.perm_y[((j + dj) & N) as usize]
                        ^ self.perm_z[((k + dk) & N) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(&mut c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2;
        }

        accum.abs()
    }

    fn perlin_interp(c: &mut [[[Vec3; 2]; 2]], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k].dot(&weight);
                }
            }
        }
        accum
    }

    fn perlin_generate_perm(buf: &mut [i32]) {
        for i in 0..buf.len() {
            buf[i] = i as i32;
        }

        Self::permute(buf);
    }

    fn permute(buf: &mut [i32]) {
        for i in (1..buf.len()).rev() {
            let target = thread_rng().gen_range(0, i);
            buf.swap(i, target);
        }
    }
}
