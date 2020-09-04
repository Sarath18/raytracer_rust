use crate::vector::Vector3;
use crate::ray::Ray;
use crate::sphere::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct Material {
  pub albedo: Vector3
}

impl Material {
  pub fn scatter(&self, _ray: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
    let scatter_direction = rec.normal + Vector3::random_unit_vector();
    *scattered = Ray{origin: rec.p, direction: scatter_direction};
    *attenuation = self.albedo;
    return true;
  }
}
