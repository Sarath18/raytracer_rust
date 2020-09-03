use crate::vector::Vector3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct ImageInfo {
  pub aspect_ratio: f64,
  pub height: u32,
  pub width: u32,
  pub samples_per_pixel: u32
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
  pub fn init() -> Self {
    Self {
      focal_length: 0.0,
      viewport_height: 0.0,
      viewport_width: 0.0,
      origin: Vector3::zero(),
      horizonal: Vector3::zero(),
      vertical: Vector3::zero(),
      lower_left_corner: Vector3::zero()
    }
  }

  pub fn get_ray(&self, u: &f64, v: &f64) -> Ray {
    return Ray {
      origin: self.origin,
      direction: self.lower_left_corner + *u * self.horizonal + *v * self.vertical - self.origin
    };
  }
}
