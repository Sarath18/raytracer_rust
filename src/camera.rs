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
  pub viewport_height: f64,
  pub viewport_width: f64,
  pub origin: Vector3,
  pub horizonal: Vector3,
  pub vertical: Vector3,
  pub lower_left_corner: Vector3
}

impl Camera {
  pub fn init(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f64, aspect_ratio: f64) -> Self {
    let theta = vfov.to_radians();
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = Vector3::unit_vector(lookfrom - lookat);
    let u = Vector3::unit_vector(vup.cross(w));
    let v = w.cross(u);

    let origin = lookfrom;
    let horizonal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizonal/2.0 - vertical/2.0 - w;
    
    return Self {
      viewport_height: viewport_height,
      viewport_width: viewport_width,
      origin: origin,
      horizonal: horizonal,
      vertical: vertical,
      lower_left_corner: lower_left_corner
    };
  }

  pub fn get_ray(&self, s: &f64, t: &f64) -> Ray {
    return Ray {
      origin: self.origin,
      direction: self.lower_left_corner + *s * self.horizonal + *t * self.vertical - self.origin
    };
  }
}
