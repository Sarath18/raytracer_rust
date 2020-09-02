use crate::vector::Vector3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
  pub p: Vector3,
  pub normal: Vector3,
  pub t: f64
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  pub center: Vector3,
  pub radius: f64
}

impl Sphere {
  pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc = ray.origin - self.center;

    let a = ray.direction.norm();
    let half_b = oc.dot(ray.direction);
    let c = oc.norm() - self.radius * self.radius;

    let discriminant = (half_b * half_b) - (a * c);
    if discriminant > 0.0 {
      let root = discriminant.sqrt();

      let mut temp = (-half_b - root) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = ray.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }

      temp = (-half_b + root) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = ray.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
    }

    return false;
  }
}
