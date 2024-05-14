pub mod obj;
pub mod r;
pub mod v;
use image::{ImageBuffer, Rgb, RgbImage};
pub use obj::HitRecord;
pub use obj::Object;
pub use r::R;
use rand::{thread_rng, Rng};
pub use v::V;

#[derive(Debug)]
pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub viewport_width: f64,
    pub viewport_height: f64,
}
impl Settings {
    pub fn new(
        image_width: u32,
        image_height: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Settings {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        Settings {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
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
        let pixel_00 = self.camera.pos
            - V(0.0, 0.0, self.camera.focal_length)
            - (viewport_u + viewport_v) / 2.0
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        let mut rng = thread_rng();

        for i in 0..settings.image_width {
            for j in 0..settings.image_height {
                let mut pixel_color = V(0.0, 0.0, 0.0);
                for _ in 1..=settings.samples_per_pixel {
                    let (offset_x, offset_y) = (rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
                    let pixel = pixel_00
                        + (i as f64 + offset_x) * pixel_delta_u
                        + (j as f64 + offset_y) * pixel_delta_v;
                    let ray = R::connect(self.camera.pos, pixel);
                    pixel_color += ray.color(&self.objects, settings.max_depth)
                }
                pixel_color /= settings.samples_per_pixel as f64;
                imgbuf.put_pixel(i, j, pixel_color.into());
            }
        }
    }
}

#[derive(Debug)]
pub struct Intvl(f64, f64);
impl Intvl {
    fn surround(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }
}
