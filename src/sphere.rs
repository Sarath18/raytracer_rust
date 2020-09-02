use crate::vector::Vector3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  pub center: Vector3,
  pub radius: f64
}

impl Sphere {
  pub fn hit_sphere(&self, ray: &Ray) -> f64 {
    let oc = ray.origin - self.center;

    let a = ray.direction.norm();
    let half_b = oc.dot(ray.direction);
    let c = oc.norm() - self.radius * self.radius;

    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
      return -1.0;
    } else {
      return (-half_b - discriminant.sqrt()) / a;
    }
  }
}