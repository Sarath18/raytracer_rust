use crate::vector::Vector3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  pub center: Vector3,
  pub radius: f64
}

impl Sphere {
  pub fn hit_sphere(&self, ray: &Ray) -> bool {
    let oc = ray.origin - self.center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = (b * b) - (4.0 * a * c);
    return discriminant > 0.0;
  }
}