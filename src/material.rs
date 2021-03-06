use crate::vector::Vector3;
use crate::ray::Ray;
use crate::sphere::HitRecord;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub enum SurfaceType {
  Diffuse,
  Reflective { fuzz: f64 },
  Refractive { ref_idx: f64 }
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
  pub albedo: Vector3,
  pub surface: SurfaceType
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
  return v - 2.0 * v.dot(n) * n;
}

pub fn refract(uv: Vector3, n: Vector3, etai_over_etat: f64) -> Vector3 {
  let cos_theta = -uv.dot(n);
  let r_out_perp = etai_over_etat * (uv + cos_theta * n);
  let r_out_parallel = -(1.0 - r_out_perp.norm()).abs().sqrt() * n;
  return r_out_perp + r_out_parallel;
}

pub fn schlick(cosine: &f64, ref_idx: &f64) -> f64 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0 * r0;
  return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
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
      // Diffuse
      SurfaceType::Diffuse => {
        let scatter_direction = rec.normal + Vector3::random_unit_vector();
        *scattered = Ray{origin: rec.p, direction: scatter_direction};
        *attenuation = self.albedo;
        return true;
      },
      // Reflective
      SurfaceType::Reflective { mut fuzz } => {
        fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        let reflected = reflect(Vector3::unit_vector(ray.direction), rec.normal);
        *scattered = Ray{origin: rec.p, direction: reflected + fuzz * Vector3::random_in_unit_sphere()};
        *attenuation = self.albedo;
        return scattered.direction.dot(rec.normal) > 0.0;
      },
      // Refractive
      SurfaceType::Refractive { ref_idx } => {
        *attenuation = Vector3::from_one(1.0);
        let etai_over_etat = if rec.front_face { 1.0 / ref_idx } else { ref_idx };

        let unit_direction = Vector3::unit_vector(ray.direction);

        let mut cos_theta = -unit_direction.dot(rec.normal);
        cos_theta = if cos_theta < 1.0 { cos_theta } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
          let reflected = reflect(unit_direction, rec.normal);
          *scattered = Ray{origin: rec.p, direction: reflected};
          return true;
        }

        let reflect_prob = schlick(&cos_theta, &etai_over_etat);

        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
          let reflected = reflect(unit_direction, rec.normal);
          *scattered = Ray{origin: rec.p, direction: reflected};
          return true;
        }

        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray{origin: rec.p, direction: refracted};
        return true;
      }
    }
  }
}
