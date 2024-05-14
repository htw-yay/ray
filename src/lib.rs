pub mod obj;
pub mod r;
pub mod v;
use image::{ImageBuffer, Rgb, RgbImage};
pub use obj::HitRecord;
pub use obj::Object;
pub use r::R;
pub use v::V;

#[derive(Debug)]
pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub viewport_width: f64,
    pub viewport_height: f64,
}
impl Settings {
    pub fn new(image_width: u32, image_height: u32) -> Settings {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        Settings {
            image_width,
            image_height,
            viewport_width,
            viewport_height,
        }
    }
    pub fn build(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        RgbImage::new(self.image_width, self.image_height)
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Camera {
    pub pos: V,
    pub focal_length: f64,
}
impl Scene {
    pub fn render(&self, settings: &Settings, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let viewport_u = V(settings.viewport_width, 0.0, 0.0);
        let viewport_v = V(0.0, -settings.viewport_height, 0.0);
        let pixel_delta_u = viewport_u / settings.image_width as f64;
        let pixel_delta_v = viewport_v / settings.image_height as f64;

        for i in 0..settings.image_width {
            for j in 0..settings.image_height {
                let pixel = self.camera.pos
                    - V(0.0, 0.0, self.camera.focal_length)
                    - (viewport_u + viewport_v) / 2.0
                    + 0.5 * (pixel_delta_u + pixel_delta_v)
                    + i as f64 * pixel_delta_u
                    + j as f64 * pixel_delta_v;
                let ray = R::connect(self.camera.pos, pixel);
                imgbuf.put_pixel(i, j, ray.color(&self.objects));
            }
        }
    }
}

#[derive(Debug)]
pub struct Intvl(f64, f64);
impl Intvl {
    fn len(&self) -> f64 {
        self.1 - self.0
    }
    fn contain(&self, x: f64) -> bool {
        self.0 <= x && x <= self.1
    }
    fn surround(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }
}
