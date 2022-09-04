use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

/// Construct a Point from a tuple
impl From<(f32, f32, f32)> for Point {
  fn from(tuple: (f32, f32, f32)) -> Point {
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
impl From<(f32, f32, f32)> for Vector {
  fn from(tuple: (f32, f32, f32)) -> Vector {
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
