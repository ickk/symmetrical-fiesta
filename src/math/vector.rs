#![allow(clippy::op_ref)]

use super::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Vector { x, y, z }
  }

  pub fn magnitude(self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn normalise(self) -> Vector {
    (1.0 / self.magnitude()) * self
  }

  pub fn dot(self, rhs: Vector) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  pub fn cross(self, rhs: Vector) -> Vector {
    Vector {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }

  pub fn reflect(self, normal: Vector) -> Vector {
    self - normal * 2.0 * self.dot(normal)
  }
}

impl ApproxEq for Vector {
  fn approx_eq(&self, rhs: Vector) -> bool {
    self.x.approx_eq(rhs.x) && self.y.approx_eq(rhs.y) && self.z.approx_eq(rhs.z)
  }
}

/// Construct a Vector from a tuple
impl From<Tuple3> for Vector {
  fn from(tuple: Tuple3) -> Vector {
    Vector {
      x: tuple.0,
      y: tuple.1,
      z: tuple.2,
    }
  }
}

/// Construct a vector by subtracting one point from another
impl Sub<Point> for Point {
  type Output = Vector;

  fn sub(self, rhs: Point) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl Add<Vector> for Vector {
  type Output = Vector;

  fn add(self, rhs: Vector) -> Vector {
    Vector {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub<Vector> for Vector {
  type Output = Vector;

  fn sub(self, rhs: Vector) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl Neg for Vector {
  type Output = Vector;

  fn neg(self) -> Vector {
    Vector {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Mul<f32> for Vector {
  type Output = Vector;

  fn mul(self, scalar: f32) -> Vector {
    Vector {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl Mul<Vector> for f32 {
  type Output = Vector;

  fn mul(self, vector: Vector) -> Vector {
    Vector {
      x: self * vector.x,
      y: self * vector.y,
      z: self * vector.z,
    }
  }
}

impl Div<f32> for Vector {
  type Output = Vector;

  fn div(self, divisor: f32) -> Vector {
    Vector {
      x: self.x / divisor,
      y: self.y / divisor,
      z: self.z / divisor,
    }
  }
}

impl Matrix4x4 {
  #[inline]
  pub fn mul_vec_unchecked(&self, vector: Vector) -> Vector {
    Vector {
      x: self[0][0] * vector.x + self[0][1] * vector.y + self[0][2] * vector.z,
      y: self[1][0] * vector.x + self[1][1] * vector.y + self[1][2] * vector.z,
      z: self[2][0] * vector.x + self[2][1] * vector.y + self[2][2] * vector.z,
    }
  }
}

impl Mul<Vector> for &Matrix4x4 {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Vector {
    assert!(0.0
      .approx_eq(self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * 0.0));

    self.mul_vec_unchecked(rhs)
  }
}

impl Mul<Vector> for Matrix4x4 {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Vector {
    &self * rhs
  }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
  use super::*;

  #[test]
  fn construct_vector() {
    // Construct a Vector from a tuple
    let vector: Vector = (1.0, 2.0, 3.0).into();
    let expected = Vector {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert!(matches!(vector, expected));

    // Construct a Vector by subtracting one point from another
    let point_a = Point {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    let point_b = Point {
      x: 0.1,
      y: 0.2,
      z: 0.3,
    };
    let vector = point_a - point_b;
    let expected = Vector {
      x: 0.9,
      y: 1.8,
      z: 2.7,
    };
    assert!(matches!(vector, expected))
  }

  #[test]
  fn negate_vector() {
    let vector = Vector::from((1.0, 2.0, 3.0));
    let expected = Vector {
      x: -1.0,
      y: -2.0,
      z: -3.0,
    };
    assert!(matches!(-vector, expected));
  }

  #[test]
  fn add_vectors() {
    let vector_a: Vector = (1.0, 2.0, 3.0).into();
    let vector_b: Vector = (0.1, 0.2, 0.3).into();
    let sum = vector_a + vector_b;
    let expected = Vector {
      x: 1.1,
      y: 2.2,
      z: 3.3,
    };
    assert!(matches!(sum, expected));
  }

  #[test]
  fn subtract_vectors() {
    let vector_a: Vector = (1.0, 2.0, 3.0).into();
    let vector_b: Vector = (0.1, 0.2, 0.3).into();
    let result = vector_a - vector_b;
    let expected = Vector {
      x: 0.9,
      y: 1.8,
      z: 2.7,
    };
    assert!(matches!(result, expected));
  }

  #[test]
  fn multiply_vector_by_scalar() {
    // multiply scalar from the left
    let scalar = 2.0;
    let vector: Vector = (0.5, 1.0, 2.0).into();
    let result = scalar * vector;
    let expected = Vector {
      x: 1.0,
      y: 2.0,
      z: 4.0,
    };
    assert!(matches!(result, expected));
    // multiple scalar on the right
    let result = vector * scalar;
    let expected = Vector {
      x: 1.0,
      y: 2.0,
      z: 4.0,
    };
    assert!(matches!(result, expected));
  }

  #[test]
  fn divide_vector_by_scalar() {
    let scalar = 2.0;
    let vector: Vector = (1.0, 2.0, 4.0).into();
    let result = vector / scalar;
    let expected = Vector {
      x: 0.5,
      y: 1.0,
      z: 2.0,
    };
    assert!(matches!(result, expected));
  }

  #[test]
  fn magnitude_of_vector() {
    let vector: Vector = (3.0, 4.0, 0.0).into();
    let magnitude = vector.magnitude();
    assert_eq!(magnitude, 5.0);
  }

  #[test]
  fn norm_of_vector() {
    let vector: Vector = (3.0, 4.0, 0.0).into();
    let norm = vector.normalise();
    let expected = Vector {
      x: 3.0 / 5.0,
      y: 4.0 / 5.0,
      z: 0.0,
    };
    assert!(matches!(norm, expected));
  }

  #[test]
  fn dot_product_of_vectors() {
    let vector_a: Vector = (1.0, 2.0, 3.0).into();
    let vector_b: Vector = (2.0, 2.0, 2.0).into();
    let dot_product = vector_a.dot(vector_b);
    assert_eq!(dot_product, 12.0);
  }

  #[test]
  fn cross_product_of_vectors() {
    let vector_a: Vector = (1.0, 2.0, 3.0).into();
    let vector_b: Vector = (2.0, 3.0, 4.0).into();
    let a_cross_b = vector_a.cross(vector_b);
    let expected = Vector {
      x: -1.0,
      y: 2.0,
      z: -1.0,
    };
    assert!(matches!(a_cross_b, expected));
    let b_cross_a = vector_b.cross(vector_a);
    let expected = Vector {
      x: 1.0,
      y: -2.0,
      z: 1.0,
    };
    assert!(matches!(b_cross_a, expected));
  }

  #[test]
  fn reflect_vector_45_degrees() {
    let vector = Vector::from((1.0, -1.0, 0.0));
    let normal = Vector::from((0.0, 1.0, 0.0));

    let result = vector.reflect(normal);
    let expected = Vector::from((1.0, 1.0, 0.0));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn reflect_vector_slanted() {
    let vector = Vector::from((0.0, -1.0, 0.0));
    let normal = Vector::from((1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt(), 0.0));

    let result = vector.reflect(normal);
    let expected = Vector::from((1.0, 0.0, 0.0));
    assert!(result.approx_eq(expected));
  }
}

#[allow(dead_code)]
mod doc_tests {
  /// ```compile_fail
  /// use raytracer_challenge::math::*;
  ///
  /// let scalar = 2.0;
  /// let vector = Vector {x: 4.0, y: 5.0, z: 6.0 };
  /// let result = scalar / vector;
  /// ```
  fn divide_scalar_by_vector_compile_fail() {}
}
