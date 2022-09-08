use super::*;

use std::ops::Mul;

pub struct Ray {
  pub origin: Point,
  pub direction: Vector,
}

impl Ray {
  pub fn new(origin: impl Into<Point>, direction: impl Into<Vector>) -> Self {
    Ray {
      origin: origin.into(),
      direction: direction.into(),
    }
  }

  pub fn position(&self, t: f32) -> Point {
    self.origin + self.direction * t
  }
}

impl ApproxEq for Ray {
  fn approx_eq(&self, rhs: Ray) -> bool {
    self.origin.approx_eq(rhs.origin) && self.direction.approx_eq(rhs.direction)
  }
}

impl Mul<&Ray> for &Matrix4x4 {
  type Output = Ray;

  fn mul(self, rhs: &Ray) -> Ray {
    Ray {
      origin: self * rhs.origin,
      direction: self * rhs.direction,
    }
  }
}

impl Mul<Ray> for &Matrix4x4 {
  type Output = Ray;

  fn mul(self, rhs: Ray) -> Ray {
    self * &rhs
  }
}

impl Mul<&Ray> for Matrix4x4 {
  type Output = Ray;

  fn mul(self, rhs: &Ray) -> Ray {
    &self * rhs
  }
}

impl Mul<Ray> for Matrix4x4 {
  type Output = Ray;

  fn mul(self, rhs: Ray) -> Ray {
    &self * &rhs
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_ray() {
    let origin = Point::from((1.0, 2.0, 3.0));
    let direction = Vector::from((4.0, 5.0, 6.0));

    let ray = Ray::new(origin, direction);

    assert!(ray.origin.approx_eq(origin));
    assert!(ray.direction.approx_eq(direction));
  }

  #[test]
  fn ray_position() {
    let ray = Ray::new((2.0, 3.0, 4.0), (1.0, 0.0, 0.0));

    let position = ray.position(0.0);
    let expected = Point::from((2.0, 3.0, 4.0));
    assert!(position.approx_eq(expected));

    let position = ray.position(1.0);
    let expected = Point::from((3.0, 3.0, 4.0));
    assert!(position.approx_eq(expected));

    let position = ray.position(-1.0);
    let expected = Point::from((1.0, 3.0, 4.0));
    assert!(position.approx_eq(expected));

    let position = ray.position(2.5);
    let expected = Point::from((4.5, 3.0, 4.0));
    assert!(position.approx_eq(expected));
  }

  #[test]
  fn translate_ray() {
    let ray = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
    let transform = Matrix4x4::translation(3.0, 4.0, 5.0);

    let result = transform * ray;
    let expected = Ray::new((4.0, 6.0, 8.0), (0.0, 1.0, 0.0));

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn scale_ray() {
    let ray = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
    let transform = Matrix4x4::scale(2.0, 3.0, 4.0);

    let result = transform * ray;
    let expected = Ray::new((2.0, 6.0, 12.0), (0.0, 3.0, 0.0));

    assert!(result.approx_eq(expected));
  }
}
