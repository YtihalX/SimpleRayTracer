use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::vec3::Point3;

const POINT_COUNT: usize = 256;
#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    fn perlin_generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
        let mut p: Vec<usize> = (0..POINT_COUNT - 1).collect();
        for i in (1..p.len()).rev() {
            let target = rng.gen_range(0..i);
            p.swap(i, target);
        }
        p
    }
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let ranfloat: Vec<f64> = (0..POINT_COUNT)
            .map(|_| rng.gen_range(0f64..1f64))
            .collect();
        let perm_x = Perlin::perlin_generate_perm(&mut rng);
        let perm_y = Perlin::perlin_generate_perm(&mut rng);
        let perm_z = Perlin::perlin_generate_perm(&mut rng);
        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        u = u * u * (3f64 - 2f64 * u);
        v = v * v * (3f64 - 2f64 * v);
        w = w * w * (3f64 - 2f64 * w);

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;
        let mut c = [[[0f64; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0f64;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f64 * u + (1 - i) as f64 * (1f64 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1f64 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1f64 - w))
                        * c[i][j][k];
                }
            }
        }
        acc
    }
}
