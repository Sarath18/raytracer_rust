use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Add for Vector3 {
  type Output = Vector3;

  fn add(self, vec: Vector3) -> Vector3 {
    Vector3 {
      x: self.x + vec.x,
      y: self.y + vec.y,
      z: self.z + vec.z
    }
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  fn sub(self, vec: Vector3) -> Vector3 {
    Vector3 {
      x: self.x - vec.x,
      y: self.y - vec.y,
      z: self.z - vec.z
    }
  }
}

impl Div<f64> for Vector3 {
  type Output = Vector3;

  fn div(self, val: f64) -> Vector3 {
    if val == 0.0 {
      return self;
    } else {
      Vector3 {
        x: self.x / val,
        y: self.y / val,
        z: self.z / val
      }
    }
  }
}

impl Neg for Vector3 {
  type Output = Vector3;

  fn neg(self) -> Vector3 {
    Vector3 {
      x: - self.x,
      y: - self.y,
      z: - self.z
    }
  }
}

impl Mul<Vector3> for Vector3 {
  type Output = Vector3;

  fn mul(self, vec: Vector3) -> Vector3 {
    Vector3 {
      x: self.x * vec.x,
      y: self.y * vec.y,
      z: self.z * vec.z
    }
  }
}

impl Mul<f64> for Vector3 {
  type Output = Vector3;

  fn mul(self, val: f64) -> Vector3 {
    Vector3 {
      x: self.x * val,
      y: self.y * val,
      z: self.z * val
    }
  }
}

impl Mul<Vector3> for f64 {
  type Output = Vector3;

  fn mul(self, vec: Vector3) -> Vector3 {
    vec * self
  }
}

impl Vector3 {
  pub fn from_one(value: f64) -> Vector3 {
    Vector3 {
      x: value,
      y: value,
      z: value
    }
  }

  pub fn length(&self) -> f64 {
    return self.norm().sqrt();
  }

  pub fn norm(&self) -> f64 {
    return self.x * self.x + self.y * self.y + self.z * self.z;
  }

  pub fn unit_vector(vec: Vector3) -> Vector3 {
    return vec / vec.length();
  }

  pub fn zero() -> Vector3 {
    return Vector3 {
      x: 0.0, 
      y: 0.0, 
      z: 0.0
    };
  }

  pub fn dot(&self, vec: Vector3) -> f64 {
    return self.x * vec.x + self.y * vec.y + self.z * vec.z;
  }
}