use crate::rtweekend::random_int_range;
use crate::vec3::Vector;

const POINT_COUNT: i32 = 256;

#[derive(Default)]
pub struct Perlin {
    randvec: Vec<Vector>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn initialise(&mut self) {
        for _i in 0..POINT_COUNT {
            self.randvec.push(Vector::random_range(-1.0, 1.0).unit());
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
        let mut c: [[[Vector; 2]; 2]; 2] = Default::default();
        for (di, a) in c.iter_mut().enumerate() {
            for (dj, b) in a.iter_mut().enumerate() {
                for (dk, d) in b.iter_mut().enumerate() {
                    *d = self.randvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }
    pub fn turb(&self, p: Vector, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
    fn perlin_interp(c: [[[Vector; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for (i, a) in c.iter().enumerate() {
            for (j, b) in a.iter().enumerate() {
                for (k, d) in b.iter().enumerate() {
                    let weight_v = Vector::new(u - (i as f64), v - (j as f64), w - (k as f64));
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * d.dot(&weight_v);
                }
            }
        }
        accum
    }
}
