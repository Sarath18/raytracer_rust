use crate::vector::Vector3;
use crate::ray::Ray;
use crate::sphere::HitRecord;

#[derive(Copy, Clone, Debug)]
pub enum SurfaceType {
  Diffuse,
  Reflective
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
  pub albedo: Vector3,
  pub surface: SurfaceType
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3{
  return v - 2.0 * v.dot(n) * n;
}

impl Material {
  pub fn init() -> Self {
    Self {
      albedo: Vector3::zero(),
      surface: SurfaceType::Diffuse
    }
  }

  pub fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
    match self.surface {
      SurfaceType::Diffuse => {
        let scatter_direction = rec.normal + Vector3::random_unit_vector();
        *scattered = Ray{origin: rec.p, direction: scatter_direction};
        *attenuation = self.albedo;
        return true;
      },
      SurfaceType::Reflective => {
        let reflected = reflect(Vector3::unit_vector(ray.direction), rec.normal);
        *scattered = Ray{origin: rec.p, direction: reflected};
        *attenuation = self.albedo;
        return scattered.direction.dot(rec.normal) > 0.0;
      }
    }
  }
}
