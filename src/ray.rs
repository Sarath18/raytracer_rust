use crate::vector::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
  pub origin: Vector3,
  pub direction: Vector3
}

impl Ray {
  fn at(&self, t: f64) -> Vector3 {
    return self.origin + t * self.direction;
  }
}