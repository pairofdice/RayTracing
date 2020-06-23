use std::ops::{AddAssign, SubAssign};
fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / (IMAGE_WIDTH-1) as f64;
            let g = (j as f64) / (IMAGE_WIDTH-1) as f64;
            let b = 0.25;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    fn x(self) -> f64 { self.e[0] }
    fn y(self) -> f64 { self.e[1] }
    fn z(self) -> f64 { self.e[2] }

    fn new() -> Vec3 {
        Vec3 {
            e: [0.0, 0.0 ,0.0],
        }
    }

    fn new3(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1 , e2],
        }
    }

    fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    fn multiply(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }

    fn divide(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] + other.x(), self.e[1] + other.y(), self.e[2] + other.z(),
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] - other.x(), self.e[1] - other.y(), self.e[2] - other.z(),
        }
    }
}