use image::io::Reader as ImageReader;

pub struct Image {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn new(path: &str) -> Self {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        let data = img.into_rgb8().into_raw();
        Self {
            width,
            height,
            data,
        }
    }

    fn clamp(&self, x: u32, min: u32, max: u32) -> u32 {
        if x < min {
            min
        } else if x < max {
            x
        } else {
            max - 1
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        let x = self.clamp(x, 0, self.width);
        let y = self.clamp(y, 0, self.height);
        let idx = (x + y * self.width) as usize * 3;
        [self.data[idx], self.data[idx + 1], self.data[idx + 2]]
    }
}
