use crate::camera::{Camera, ImageInfo};
use crate::sphere::{Sphere, HitRecord};
use crate::ray::Ray;

pub struct World {
  pub spheres: Vec<Sphere>
}

impl World {
  pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for sphere in self.spheres.iter() {
      let mut temp_rec = HitRecord::init();
      if sphere.hit(ray, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec;
      }
    }
    return hit_anything;
  }
}

// #[derive(Copy, Clone, Debug)]
pub struct Scene {
  pub image_info: ImageInfo,
  pub camera: Camera,
  pub world: World
}
