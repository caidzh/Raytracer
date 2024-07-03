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
        let i = ((4.0 * p.x) as i32) & 255;
        let j = ((4.0 * p.y) as i32) & 255;
        let k = ((4.0 * p.z) as i32) & 255;
        self.randfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}
