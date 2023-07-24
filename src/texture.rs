use std::{path::Path, rc::Rc};

use image::{ImageBuffer, Rgb};

use crate::{
    perlin::Perlin,
    rtw::clamp,
    vec3::{Color, Vec3},
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn from_color(color_value: Color) -> Self {
        Self { color_value }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn from_color(c1: Color, c2: Color) -> Self {
        Self {
            odd: Rc::new(SolidColor::from_color(c2)),
            even: Rc::new(SolidColor::from_color(c1)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let sine = (10f64 * p.x()).sin() * (10f64 * p.y()).sin() * (10f64 * p.z()).sin();
        if sine < 0f64 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        Color::new(1f64, 1f64, 1f64) * self.noise.noise(&(*p * self.scale))
    }
}

pub struct ImageTexture {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageTexture {
    pub fn new(file: &Path) -> Self {
        let data = image::io::Reader::open(file)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();
        Self { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Color {
        let u = clamp(u, 0f64, 1f64);
        let v = 1f64 - clamp(v, 0f64, 1f64);
        let width = self.data.width();
        let height = self.data.height();
        let mut i = (u * width as f64) as u32;
        let mut j = (v * height as f64) as u32;
        if i >= width {
            i = width - 1
        }
        if j >= height {
            j = height - 1
        }
        let a = Color::from_rgb8(*self.data.get_pixel(i, j));
        a
    }
}
