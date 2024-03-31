use crate::{Color, Image, Interval, Point3, Perlin};


pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    inv_size: f64,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, size: f64) -> Self {
        Self {
            odd,
            even,
            inv_size: 1.0 / size,
        }
    }

    pub fn new_with_color(c1: Color, c2: Color, size: f64) -> Self {
        Self {
            odd: Box::new(SolidColor::new(c1)),
            even: Box::new(SolidColor::new(c2)),
            inv_size: 1.0 / size,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_index = (self.inv_size * p.x()).floor() as i32;
        let y_index = (self.inv_size * p.y()).floor() as i32;
        let z_index = (self.inv_size * p.z()).floor() as i32;
        let is_even = (x_index + y_index + z_index) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: Image
}


impl ImageTexture {
    pub fn new(image_path: &str) -> Self {
        Self {
            image: Image::new(image_path)
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (u * self.image.width as f64) as u32;
        let j = (v * self.image.height as f64) as u32;
        let pixel = self.image.get_pixel(i, j);
        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let s = self.scale * *p;
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z() + 10.0 * self.noise.turb(&s, 7)).sin())
    }
}