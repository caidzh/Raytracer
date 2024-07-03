use crate::rtweekend::random_double;
use crate::rtweekend::random_int_range;
use crate::vec3::Vector;

const POINT_COUNT: i32 = 256;

#[derive(Default)]
pub struct Perlin {
    randfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn initialise(&mut self) {
        for _i in 0..POINT_COUNT {
            self.randfloat.push(random_double());
        }
        Perlin::perlin_generate_perm(&mut self.perm_x);
        Perlin::perlin_generate_perm(&mut self.perm_y);
        Perlin::perlin_generate_perm(&mut self.perm_z);
    }
    fn perlin_generate_perm(a: &mut Vec<i32>) {
        for i in 0..POINT_COUNT {
            a.push(i);
        }
        Self::permute(a, POINT_COUNT);
    }
    fn permute(p: &mut [i32], n: i32) {
        for i in (1..n).rev() {
            let target = random_int_range(0, i as u32);
            p.swap(i as usize, target as usize);
        }
    }
    pub fn noise(&self, p: Vector) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = Default::default();
        for (di, a) in c.iter_mut().enumerate() {
            for (dj, b) in a.iter_mut().enumerate() {
                for (dk, d) in b.iter_mut().enumerate() {
                    *d = self.randfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum: f64 = 0.0;
        for (i, a) in c.iter().enumerate() {
            for (j, b) in a.iter().enumerate() {
                for (k, d) in b.iter().enumerate() {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * d;
                }
            }
        }
        accum
    }
}
