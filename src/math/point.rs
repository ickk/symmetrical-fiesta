use super::*;

use std::ops::{Add, Sub};

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

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
  use super::*;

  #[test]
  fn construct_point() {
    // Construct a point from a tuple
    let point: Point = (1.0, 2.0, 3.0).into();
    let expected = Point {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    };
    assert!(matches!(point, expected));
  }

  #[test]
  fn add_vector_to_point() {
    // Add a Vector to a Point
    let point: Point = (1.0, 2.0, 3.0).into();
    let vector: Vector = (1.5, 2.5, 3.5).into();
    let sum = point + vector;
    let expected = Point {
      x: 2.5,
      y: 4.5,
      z: 6.5,
    };
    assert!(matches!(sum, expected));

    // Add a Point to a Vector
    let sum = vector + point;
    let expected = Point {
      x: 2.5,
      y: 4.5,
      z: 6.5,
    };
    assert!(matches!(sum, expected));
  }

  #[test]
  fn subtract_vector_from_point() {
    let point: Point = (1.0, 2.0, 3.0).into();
    let vector: Vector = (1.5, 2.5, 3.5).into();
    let result = point - vector;
    let expected = Point {
      x: -0.5,
      y: -0.5,
      z: -0.5,
    };
    assert!(matches!(result, expected));
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
}