use super::*;

impl Matrix4x4 {
  pub fn translation(x: f32, y: f32, z: f32) -> Self {
    Matrix4x4 {
      inner: [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn scale(x: f32, y: f32, z: f32) -> Self {
    Matrix4x4 {
      inner: [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn rotation_x(angle: f32) -> Self {
    let (sin_r, cos_r) = angle.sin_cos();
    Matrix4x4 {
      inner: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos_r, -sin_r, 0.0],
        [0.0, sin_r, cos_r, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn rotation_y(angle: f32) -> Self {
    let (sin_r, cos_r) = angle.sin_cos();
    Matrix4x4 {
      inner: [
        [cos_r, 0.0, sin_r, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin_r, 0.0, cos_r, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn rotation_z(angle: f32) -> Self {
    let (sin_r, cos_r) = angle.sin_cos();
    Matrix4x4 {
      inner: [
        [cos_r, -sin_r, 0.0, 0.0],
        [sin_r, cos_r, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }

  pub fn shear(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Self {
    Matrix4x4 {
      inner: [
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::PI;

  #[test]
  fn translation_matrix_applied_to_point() {
    let transform = Matrix4x4::translation(5.0, -3.0, 2.0);
    let point: Point = (-3.0, 4.0, 5.0).into();

    let result = transform * point;
    let expected = Point::from((2.0, 1.0, 7.0));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn inverse_translation_matrix_applied_to_point() {
    let transform = Matrix4x4::translation(5.0, -3.0, 2.0);
    let inverse_transform = transform.inverse().unwrap();
    let point: Point = (-3.0, 4.0, 5.0).into();

    let result = inverse_transform * point;
    let expected = Point::from((-8.0, 7.0, 3.0));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn translation_matrix_applied_to_vector() {
    // should be functionally a no-op
    let transform = Matrix4x4::translation(5.0, -3.0, 2.0);
    let vector = Vector::from((-3.0, 4.0, 5.0));

    let result = transform * vector;
    assert!(result.approx_eq(vector));
  }

  #[test]
  fn scale_matrix_applied_to_point() {
    let transform = Matrix4x4::scale(2.0, 3.0, 4.0);
    let point: Point = (-4.0, 6.0, 8.0).into();

    let result = transform * point;
    let expected: Point = (-8.0, 18.0, 32.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn scale_matrix_applied_to_vector() {
    let transform = Matrix4x4::scale(2.0, 3.0, 4.0);
    let vector: Vector = (-4.0, 6.0, 8.0).into();

    let result = transform * vector;
    let expected: Vector = (-8.0, 18.0, 32.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn inverse_scale_matrix_applied_to_vector() {
    let transform = Matrix4x4::scale(2.0, 3.0, 4.0);
    let inverse_transform = transform.inverse().unwrap();
    let vector: Vector = (-4.0, 6.0, 8.0).into();

    let result = inverse_transform * vector;
    let expected: Vector = (-2.0, 2.0, 2.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn reflection_matrix_applied_to_point() {
    let transform = Matrix4x4::scale(-1.0, 1.0, 1.0);
    let point = Point::from((2.0, 3.0, 4.0));

    let result = transform * point;
    let expected: Point = (-2.0, 3.0, 4.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn rotation_about_x() {
    let point: Point = (0.0, 1.0, 0.0).into();

    let eighth_turn = Matrix4x4::rotation_x(PI / 4.0);
    let result = eighth_turn * point;
    let expected = Point::from((0.0, 1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt()));
    assert!(result.approx_eq(expected));

    let quarter_turn = Matrix4x4::rotation_x(PI / 2.0);
    let result = quarter_turn * point;
    let expected = Point::from((0.0, 0.0, 1.0));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn inverse_rotation_about_x() {
    let point: Point = (0.0, 1.0, 0.0).into();

    let eighth_turn = Matrix4x4::rotation_x(PI / 4.0);
    let inverse_eighth_turn = eighth_turn.inverse().unwrap();

    let result = inverse_eighth_turn * point;
    let expected = Point::from((0.0, 1.0 / 2.0f32.sqrt(), -1.0 / 2.0f32.sqrt()));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn rotation_about_y() {
    let point: Point = (0.0, 0.0, 1.0).into();

    let eighth_turn = Matrix4x4::rotation_y(PI / 4.0);
    let result = eighth_turn * point;
    let expected = Point::from((1.0 / 2.0f32.sqrt(), 0.0, 1.0 / 2.0f32.sqrt()));
    assert!(result.approx_eq(expected));

    let quarter_turn = Matrix4x4::rotation_y(PI / 2.0);
    let result = quarter_turn * point;
    let expected = Point::from((1.0, 0.0, 0.0));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn rotation_about_z() {
    let point: Point = (0.0, 1.0, 0.0).into();

    let eighth_turn = Matrix4x4::rotation_z(PI / 4.0);
    let result = eighth_turn * point;
    let expected = Point::from((-1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt(), 0.0));
    assert!(result.approx_eq(expected));

    let quarter_turn = Matrix4x4::rotation_z(PI / 2.0);
    let result = quarter_turn * point;
    let expected = Point::from((-1.0, 0.0, 0.0));
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn shear_x_proportion_to_y() {
    let point: Point = (2.0, 3.0, 4.0).into();
    let transform = Matrix4x4::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let result = transform * point;
    let expected: Point = (5.0, 3.0, 4.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn shear_x_proportion_to_z() {
    let point: Point = (2.0, 3.0, 4.0).into();
    let transform = Matrix4x4::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

    let result = transform * point;
    let expected: Point = (6.0, 3.0, 4.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn shear_y_proportion_to_x() {
    let point: Point = (2.0, 3.0, 4.0).into();
    let transform = Matrix4x4::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

    let result = transform * point;
    let expected: Point = (2.0, 5.0, 4.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn shear_y_proportion_to_z() {
    let point: Point = (2.0, 3.0, 4.0).into();
    let transform = Matrix4x4::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    let result = transform * point;
    let expected: Point = (2.0, 7.0, 4.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn shear_z_proportion_to_x() {
    let point: Point = (2.0, 3.0, 4.0).into();
    let transform = Matrix4x4::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

    let result = transform * point;
    let expected: Point = (2.0, 3.0, 6.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn shear_z_proportion_to_y() {
    let point: Point = (2.0, 3.0, 4.0).into();
    let transform = Matrix4x4::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    let result = transform * point;
    let expected: Point = (2.0, 3.0, 7.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn sequential_transformations() {
    let point: Point = (1.0, 0.0, 1.0).into();
    let a = Matrix4x4::rotation_x(PI / 2.0);
    let b = Matrix4x4::scale(5.0, 5.0, 5.0);
    let c = Matrix4x4::translation(10.0, 5.0, 7.0);

    let point_2 = a * point;
    let expected = Point::from((1.0, -1.0, 0.0));
    assert!(point_2.approx_eq(expected));

    let point_3 = b * point_2;
    let expected = Point::from((5.0, -5.0, 0.0));
    assert!(point_3.approx_eq(expected));

    let point_4 = c * point_3;
    let expected = Point::from((15.0, 0.0, 7.0));
    assert!(point_4.approx_eq(expected));
  }

  #[test]
  fn chained_transformations() {
    let point = Point::from((1.0, 0.0, 1.0));
    let a = Matrix4x4::rotation_x(PI / 2.0);
    let b = Matrix4x4::scale(5.0, 5.0, 5.0);
    let c = Matrix4x4::translation(10.0, 5.0, 7.0);

    let transform = c * b * a;
    let result = transform * point;
    let expected = Point::from((15.0, 0.0, 7.0));

    assert!(result.approx_eq(expected));
  }
}
