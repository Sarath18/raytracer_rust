use crate::vector::Vector3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
  pub p: Vector3,
  pub normal: Vector3,
  pub t: f64,
  pub front_face: bool,
  pub mat: Material
}

impl HitRecord {
  pub fn init() -> Self {
    Self {
      p: Vector3::zero(),
      normal: Vector3::zero(),
      t: 0.0,
      front_face: false,
      mat: Material::init()
    }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
    self.front_face = ray.direction.dot(outward_normal) < 0.0;
    self.normal = if self.front_face { outward_normal } else { -outward_normal }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
  pub center: Vector3,
  pub radius: f64,
  pub mat: Material
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = self.mat;
        return true;
      }

      temp = (-half_b + root) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = self.mat;
        return true;
      }
    }

    return false;
  }
}
