use crate::utils::random;
use crate::{core::Vec3, core::Texture};

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    #[allow(dead_code)]
    pub fn new(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f32::sin(self.scale * p.x + 5.0 * self.noise.turb(&p, 7)))
    }
}

#[allow(dead_code)]
fn perlin_generate() -> Vec<Vec3> {
    let mut p = Vec::with_capacity(256);
    for _ in 0..256 {
        p.push(
            Vec3::new(
                -1.0 + 2.0 * random::f32(),
                -1.0 + 2.0 * random::f32(),
                -1.0 + 2.0 * random::f32(),
            )
            .normalize(),
        );
    }
    p
}

#[allow(dead_code)]
fn permute(p: &mut [usize], n: usize) {
    for i in (0..n as usize).rev() {
        let target = random::usize(0..(i + 1));
        p.swap(i, target);
    }
}

#[allow(dead_code)]
fn perlin_generate_perm() -> Vec<usize> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i);
    }
    permute(&mut p, 256);
    p
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                    * c[i][j][k].dot(&weight);
            }
        }
    }
    accum
}

#[derive(Clone)]
pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ran_vec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x - f32::floor(p.x);
        let v = p.y - f32::floor(p.y);
        let w = p.z - f32::floor(p.z);
        let i = f32::floor(p.x) as usize;
        let j = f32::floor(p.y) as usize;
        let k = f32::floor(p.z) as usize;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                }
            }
        }
        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        f32::abs(accum)
    }
}
