use crate::rtweekend;
use std::ops::Mul;

#[derive(Default)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        let mut val: Matrix = Default::default();
        for _i in 0..=width {
            let mut row: Vec<f64> = Default::default();
            for _j in 0..=height {
                row.push(0.0);
            }
            val.data.push(row);
        }
        val
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let dx = other.data.len();
        let dy = other.data[0].len();
        let mut res: Vec<Vec<f64>> = Default::default();
        for (i, val) in res.iter_mut().enumerate().take((self.data.len() - dx) + 1) {
            for j in 0..=self.data[0].len() - dy {
                let mut sum: f64 = 0.0;
                for k in 0..dx {
                    for k_ in 0..dy {
                        sum += self.data[i + k][j + k_] * other.data[k][k_];
                    }
                }
                val.push(sum);
            }
        }
        Matrix { data: res }
    }
}

#[derive(Default)]
pub struct Sobel {
    pub gx: Matrix,
    pub gy: Matrix,
}

impl Sobel {
    pub fn new() -> Self {
        let mut val: Sobel = Default::default();
        let row: Vec<f64> = vec![-1.0,0.0,1.0];
        val.gx.data.push(row);
        let row: Vec<f64> = vec![-2.0,0.0,2.0];
        val.gx.data.push(row);
        let row: Vec<f64> = vec![-1.0,0.0,1.0];
        val.gx.data.push(row);
        let row: Vec<f64> = vec![-1.0,-2.0,-1.0];
        val.gy.data.push(row);
        let row: Vec<f64> = vec![0.0,0.0,0.0];
        val.gy.data.push(row);
        let row: Vec<f64> = vec![1.0,2.0,1.0];
        val.gy.data.push(row);
        val
    }
}

#[derive(Default)]
pub struct Canny {
    pub gaussian_kernel: Matrix,
    pub length: u32,
    pub sigma: f64,
    pub sobel: Sobel,
}

impl Canny {
    pub fn new() -> Self {
        let mut val: Canny = Canny{length:5,sigma:1.6,..Default::default()};
        let k_ = 2.0 * rtweekend::PI * val.sigma * val.sigma;
        let k = val.length / 2;
        for _i in 0..val.length {
            let mut row: Vec<f64> = Default::default();
            for _j in 0..val.length {
                row.push(0.0);
            }
            val.gaussian_kernel.data.push(row);
        }
        let mut sum: f64 = 0.0;
        for i in 0..val.length {
            for j in 0..val.length {
                val.gaussian_kernel.data[i as usize][j as usize] =
                    (((i - k) * (i - k) + (j - k) * (j - k)) as f64
                        / (2.0 * val.sigma * val.sigma))
                        .exp()
                        / k_;
                sum += val.gaussian_kernel.data[i as usize][j as usize];
            }
        }
        for i in 0..val.length {
            for j in 0..val.length {
                val.gaussian_kernel.data[i as usize][j as usize] /= sum;
            }
        }
        val.sobel = Sobel::new();
        val
    }
}
