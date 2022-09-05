use std::ops::{Add, Sub, Mul, Div, Neg};

type Tuple3 = (f32, f32, f32);

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

/// Construct a Point from a tuple
impl From<Tuple3> for Point {
  fn from(tuple: Tuple3) -> Point {
    Point {
      x: tuple.0,
      y: tuple.1,
      z: tuple.2,
    }
  }
}

impl Add<Vector> for Point {
  type Output = Point;

  fn add(self, rhs: Vector) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Add<Point> for Vector {
  type Output = Point;

  fn add(self, rhs: Point) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub<Vector> for Point {
  type Output = Point;

  fn sub(self, rhs: Vector) -> Point {
    Point {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32,
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

impl Vector {
  pub fn magnitude(self) -> f32 {
    (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
  }

  pub fn norm(self) -> Vector {
    (1.0 / self.magnitude()) * self
  }

  pub fn dot(self, rhs: Vector) -> f32 {
    self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
  }

  pub fn cross(self, rhs: Vector) -> Vector {
    Vector {
      x: self.y*rhs.z - self.z*rhs.y,
      y: self.z*rhs.x - self.x*rhs.z,
      z: self.x*rhs.y - self.y*rhs.x,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_point() {
    // Construct a point from a tuple
    let point: Point = (1.0, 2.0, 3.0).into();
    assert!(matches!(point, Point {x: 1.0, y: 2.0, z: 3.0}));
  }

  #[test]
  fn add_vector_to_point() {
    // Add a Vector to a Point
    let point: Point = (1.0, 2.0, 3.0).into();
    let vector: Vector = (1.5, 2.5, 3.5).into();
    let sum = point + vector;
    assert!(matches!(sum, Point {x: 2.5, y: 4.5, z: 6.5}));

    // Add a Point to a Vector
    let sum = vector + point;
    assert!(matches!(sum, Point {x: 2.5, y: 4.5, z: 6.5}));
  }

  #[test]
  fn subtract_vector_from_point() {
    let point: Point = (1.0, 2.0, 3.0).into();
    let vector: Vector = (1.5, 2.5, 3.5).into();
    let result = point - vector;
    assert!(matches!(result, Point {x: -0.5, y: -0.5, z: -0.5}));
  }

  #[test]
  fn construct_vector() {
    // Construct a Vector from a tuple
    let vector: Vector = (1.0, 2.0, 3.0).into();
    assert!(matches!(vector, Vector {x: 1.0, y: 2.0, z: 3.0}));

    // Construct a Vector by subtracting one point from another
    let point_a = Point { x: 1.0, y: 2.0, z: 3.0 };
    let point_b = Point { x: 0.1, y: 0.2, z: 0.3 };
    let vector = point_a - point_b;
    assert!(matches!(vector, Vector {x: 0.9, y: 1.8, z: 2.7}))
  }

  #[test]
  fn negate_vector() {
    let vector = Vector::from((1.0, 2.0, 3.0));
    assert!(matches!(-vector, Vector {x: -1.0, y: -2.0, z: -3.0}));
  }

  #[test]
  fn add_vectors() {
    let vector_a: Vector = (1.0, 2.0, 3.0).into();
    let vector_b: Vector = (0.1, 0.2, 0.3).into();
    let sum = vector_a + vector_b;
    assert!(matches!(sum, Vector {x: 1.1, y: 2.2, z: 3.3}));
  }

  #[test]
  fn subtract_vectors() {
    let vector_a: Vector = (1.0, 2.0, 3.0).into();
    let vector_b: Vector = (0.1, 0.2, 0.3).into();
    let result = vector_a - vector_b;
    assert!(matches!(result, Vector {x: 0.9, y: 1.8, z: 2.7}));
  }

  #[test]
  fn multiply_vector_by_scalar() {
    // multiply scalar from the left
    let scalar = 2.0;
    let vector: Vector = (0.5, 1.0, 2.0).into();
    let result = scalar * vector;
    assert!(matches!(result, Vector {x: 1.0, y: 2.0, z: 4.0}));
    // multiple scalar on the right
    let result = vector * scalar;
    assert!(matches!(result, Vector {x: 1.0, y: 2.0, z: 4.0}));
  }

  #[test]
  fn divide_vector_by_scalar() {
    let scalar = 2.0;
    let vector: Vector = (1.0, 2.0, 4.0).into();
    let result = vector / scalar;
    assert!(matches!(result, Vector {x: 0.5, y: 1.0, z: 2.0}));
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
    let norm = vector.norm();
    let expected = Vector {x: 3.0/5.0, y: 4.0/5.0, z: 0.0};
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
    assert!(matches!(a_cross_b, Vector {x: -1.0, y: 2.0, z: -1.0}));
    let b_cross_a = vector_b.cross(vector_a);
    assert!(matches!(b_cross_a, Vector {x: 1.0, y: -2.0, z: 1.0}));
  }
}

#[allow(dead_code)]
mod doc_tests {
  /// ```compile_fail
  /// use raytracer_challenge::math::*;
  ///
  /// let a = Point {x: 1.0, y: 2.0, z: 3.0 };
  /// let b = Point {x: 4.0, y: 5.0, z: 6.0 };
  /// let c = a + b;
  /// ```
  fn add_points_compile_fail() {}

  /// ```compile_fail
  /// use raytracer_challenge::math::*;
  ///
  /// let point = Point {x: 1.0, y: 2.0, z: 3.0 };
  /// let vector = Vector {x: 4.0, y: 5.0, z: 6.0 };
  /// let result = vector - point;
  /// ```
  fn subtract_point_from_vector_compile_fail() {}

  /// ```compile_fail
  /// use raytracer_challenge::math::*;
  ///
  /// let scalar = 2.0;
  /// let vector = Vector {x: 4.0, y: 5.0, z: 6.0 };
  /// let result = scalar / vector;
  /// ```
  fn divide_scalar_by_vector_compile_fail() {}
}
