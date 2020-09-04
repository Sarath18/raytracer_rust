use crate::vector::Vector3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct ImageInfo {
  pub aspect_ratio: f64,
  pub height: u32,
  pub width: u32,
  pub samples_per_pixel: u32,
  pub max_depth: i32
}

#[derive(Copy, Clone, Debug)]
pub struct Camera {
  pub focal_length: f64,
  pub viewport_height: f64,
  pub viewport_width: f64,
  pub origin: Vector3,
  pub horizonal: Vector3,
  pub vertical: Vector3,
  pub lower_left_corner: Vector3
}

impl Camera {
  pub fn init(vfov: f64, aspect_ratio: f64) -> Self {
    let theta = vfov.to_radians();
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;
    let origin = Vector3::zero();
    let focal_length = 1.0;
    let horizonal = Vector3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vector3{x: 0.0, y: viewport_height, z: 0.0};
    let viewport_height = 2.0;
    let lower_left_corner = origin - horizonal/2.0 - vertical/2.0 - Vector3{x: 0.0, y: 0.0, z: focal_length};
    
    return Self {
      focal_length: focal_length,
      viewport_height: viewport_height,
      viewport_width: viewport_width,
      origin: origin,
      horizonal: horizonal,
      vertical: vertical,
      lower_left_corner: lower_left_corner
    };
  }

  pub fn get_ray(&self, u: &f64, v: &f64) -> Ray {
    return Ray {
      origin: self.origin,
      direction: self.lower_left_corner + *u * self.horizonal + *v * self.vertical - self.origin
    };
  }
}
