use super::*;
use std::ops::{Index, IndexMut, Mul};

// Matrix 2x2
#[derive(Debug, Clone)]
pub struct Matrix2x2 {
  pub(super) inner: Array2x2,
}

impl From<Array2x2> for Matrix2x2 {
  fn from(array: Array2x2) -> Matrix2x2 {
    Matrix2x2 { inner: array }
  }
}

impl Index<usize> for Matrix2x2 {
  type Output = Array2;

  fn index(&self, index: usize) -> &Array2 {
    &self.inner[index]
  }
}

impl IndexMut<usize> for Matrix2x2 {
  fn index_mut(&mut self, index: usize) -> &mut Array2 {
    &mut self.inner[index]
  }
}

impl Index<usize> for &mut Matrix2x2 {
  type Output = Array2;

  fn index(&self, index: usize) -> &Array2 {
    &self.inner[index]
  }
}

impl IndexMut<usize> for &mut Matrix2x2 {
  fn index_mut(&mut self, index: usize) -> &mut Array2 {
    &mut self.inner[index]
  }
}

impl ApproxEq for Matrix2x2 {
  fn approx_eq(&self, rhs: Matrix2x2) -> bool {
    for y in 0..self.inner.len() {
      for x in 0..self.inner[0].len() {
        if !self[x][y].approx_eq(rhs[x][y]) {
          return false;
        }
      }
    }
    true
  }
}

impl Matrix2x2 {
  pub const IDENTITY: Self = Matrix2x2 {
    inner: [[1.0, 0.0], [0.0, 1.0]],
  };

  pub const ZEROS: Self = Matrix2x2 {
    inner: [[0.0; 2]; 2],
  };

  pub fn transpose(&self) -> Matrix2x2 {
    Matrix2x2 {
      inner: [[self[0][0], self[1][0]], [self[0][1], self[1][1]]],
    }
  }

  pub fn determinant(&self) -> f32 {
    self[0][0] * self[1][1] - self[0][1] * self[1][0]
  }
}

// Matrix3x3
#[derive(Debug, Clone)]
pub struct Matrix3x3 {
  pub(super) inner: Array3x3,
}

impl From<Array3x3> for Matrix3x3 {
  fn from(array: Array3x3) -> Matrix3x3 {
    Matrix3x3 { inner: array }
  }
}

impl Index<usize> for Matrix3x3 {
  type Output = Array3;

  fn index(&self, index: usize) -> &Array3 {
    &self.inner[index]
  }
}

impl IndexMut<usize> for Matrix3x3 {
  fn index_mut(&mut self, index: usize) -> &mut Array3 {
    &mut self.inner[index]
  }
}

impl Index<usize> for &mut Matrix3x3 {
  type Output = Array3;

  fn index(&self, index: usize) -> &Array3 {
    &self.inner[index]
  }
}

impl IndexMut<usize> for &mut Matrix3x3 {
  fn index_mut(&mut self, index: usize) -> &mut Array3 {
    &mut self.inner[index]
  }
}

impl ApproxEq for Matrix3x3 {
  fn approx_eq(&self, rhs: Matrix3x3) -> bool {
    for y in 0..self.inner.len() {
      for x in 0..self.inner[0].len() {
        if !self[x][y].approx_eq(rhs[x][y]) {
          return false;
        }
      }
    }
    true
  }
}

impl Matrix3x3 {
  pub const IDENTITY: Self = Matrix3x3 {
    inner: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
  };

  pub const ZEROS: Self = Matrix3x3 {
    inner: [[0.0; 3]; 3],
  };

  pub fn transpose(&self) -> Matrix3x3 {
    Matrix3x3 {
      inner: [
        [self[0][0], self[1][0], self[2][0]],
        [self[0][1], self[1][1], self[2][1]],
        [self[0][2], self[1][2], self[2][2]],
      ],
    }
  }

  /// Return the submatrix with the specified row and column removed
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
    let mut output = Matrix2x2::ZEROS;
    let mut x = 0;
    let mut y;
    for r in 0..self.inner.len() {
      if r != row {
        y = 0;
        for c in 0..self.inner.len() {
          if c != column {
            output[x][y] = self[r][c];
            y += 1;
          }
        }
        x += 1;
      }
    }
    output
  }

  pub fn minor(&self, row: usize, column: usize) -> f32 {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f32 {
    self.minor(row, column) * ((1 - ((row + column) as isize % 2) * 2) as f32)
  }

  pub fn determinant(&self) -> f32 {
    self[0][0] * self.cofactor(0, 0)
      + self[0][1] * self.cofactor(0, 1)
      + self[0][2] * self.cofactor(0, 2)
  }
}

// Matrix 4x4
#[derive(Debug, Clone)]
pub struct Matrix4x4 {
  pub(super) inner: Array4x4,
}

impl From<Array4x4> for Matrix4x4 {
  fn from(array: Array4x4) -> Matrix4x4 {
    Matrix4x4 { inner: array }
  }
}

impl Index<usize> for Matrix4x4 {
  type Output = Array4;

  fn index(&self, index: usize) -> &Array4 {
    &self.inner[index]
  }
}

impl IndexMut<usize> for Matrix4x4 {
  fn index_mut(&mut self, index: usize) -> &mut Array4 {
    &mut self.inner[index]
  }
}

impl Index<usize> for &mut Matrix4x4 {
  type Output = Array4;

  fn index(&self, index: usize) -> &Array4 {
    &self.inner[index]
  }
}

impl IndexMut<usize> for &mut Matrix4x4 {
  fn index_mut(&mut self, index: usize) -> &mut Array4 {
    &mut self.inner[index]
  }
}

impl ApproxEq for Matrix4x4 {
  fn approx_eq(&self, rhs: Matrix4x4) -> bool {
    for y in 0..self.inner.len() {
      for x in 0..self.inner[0].len() {
        if !self[x][y].approx_eq(rhs[x][y]) {
          return false;
        }
      }
    }
    true
  }
}

impl Mul<&Matrix4x4> for &Matrix4x4 {
  type Output = Matrix4x4;

  fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
    let mut output = Matrix4x4::ZEROS;

    for r in 0..4 {
      for c in 0..4 {
        output[r][c] = self[r][0] * rhs[0][c]
          + self[r][1] * rhs[1][c]
          + self[r][2] * rhs[2][c]
          + self[r][3] * rhs[3][c];
      }
    }

    output
  }
}

impl Mul<&Matrix4x4> for Matrix4x4 {
  type Output = Matrix4x4;

  fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
    &self * rhs
  }
}

impl Mul<Matrix4x4> for &Matrix4x4 {
  type Output = Matrix4x4;

  fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
    self * &rhs
  }
}

impl Mul<Matrix4x4> for Matrix4x4 {
  type Output = Matrix4x4;

  fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
    &self * &rhs
  }
}

impl Matrix4x4 {
  pub const IDENTITY: Self = Matrix4x4 {
    inner: [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ],
  };

  pub const ZEROS: Self = Matrix4x4 {
    inner: [[0.0; 4]; 4],
  };

  pub fn transpose(&self) -> Matrix4x4 {
    Matrix4x4 {
      inner: [
        [self[0][0], self[1][0], self[2][0], self[3][0]],
        [self[0][1], self[1][1], self[2][1], self[3][1]],
        [self[0][2], self[1][2], self[2][2], self[3][2]],
        [self[0][3], self[1][3], self[2][3], self[3][3]],
      ],
    }
  }

  /// Return the submatrix with the specified row and column removed
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix3x3 {
    let mut output = Matrix3x3::ZEROS;
    let mut x = 0;
    let mut y;
    for r in 0..self.inner.len() {
      if r != row {
        y = 0;
        for c in 0..self.inner.len() {
          if c != column {
            output[x][y] = self[r][c];
            y += 1;
          }
        }
        x += 1;
      }
    }
    output
  }

  pub fn minor(&self, row: usize, column: usize) -> f32 {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f32 {
    self.minor(row, column) * ((1 - ((row + column) as isize % 2) * 2) as f32)
  }

  pub fn determinant(&self) -> f32 {
    self[0][0] * self.cofactor(0, 0)
      + self[0][1] * self.cofactor(0, 1)
      + self[0][2] * self.cofactor(0, 2)
      + self[0][3] * self.cofactor(0, 3)
  }

  pub fn invertible(&self) -> bool {
    !self.determinant().approx_eq(0.0)
  }

  #[allow(clippy::result_unit_err)]
  pub fn inverse(&self) -> Result<Matrix4x4, ()> {
    let determinant = self.determinant();
    if determinant.approx_eq(0.0) {
      return Err(());
    }
    let mut inverse = Matrix4x4::ZEROS;

    for row in 0..self.inner.len() {
      for col in 0..self.inner.len() {
        inverse[col][row] = self.cofactor(row, col) / determinant;
      }
    }

    Ok(inverse)
  }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
  use super::*;

  #[test]
  fn construct_matrix2x2() {
    let matrix: Matrix2x2 = [
      [-3.0,  5.0],
      [ 1.0, -2.0]
    ].into();

    assert!(matrix[0][0].approx_eq(-3.0));
    assert!(matrix[0][1].approx_eq(5.0));
    assert!(matrix[1][0].approx_eq(1.0));
    assert!(matrix[1][1].approx_eq(-2.0));
  }

  #[test]
  fn construct_matrix3x3() {
    let matrix: Matrix3x3 = [
      [-3.0,  5.0,  0.0],
      [ 1.0, -2.0, -7.0],
      [ 0.0,  1.0,  1.0]
    ].into();

    assert!(matrix[0][0].approx_eq(-3.0));
    assert!(matrix[1][1].approx_eq(-2.0));
    assert!(matrix[2][2].approx_eq(1.0));
  }

  #[test]
  fn construct_matrix4x4() {
    let matrix: Matrix4x4 = [
      [ 1.0,  2.0,  3.0,  4.0],
      [ 5.5,  6.5,  7.5,  8.5],
      [ 9.0, 10.0, 11.0, 12.0],
      [13.5, 14.5, 15.5, 16.5],
    ].into();

    assert!(matrix[0][0].approx_eq(1.0));
    assert!(matrix[0][3].approx_eq(4.0));
    assert!(matrix[1][0].approx_eq(5.5));
    assert!(matrix[1][2].approx_eq(7.5));
    assert!(matrix[2][2].approx_eq(11.0));
    assert!(matrix[3][0].approx_eq(13.5));
    assert!(matrix[3][2].approx_eq(15.5));

    // Reference
    let matrix: &Matrix4x4 = &[
      [ 1.0,  2.0,  3.0,  4.0],
      [ 5.5,  6.5,  7.5,  8.5],
      [ 9.0, 10.0, 11.0, 12.0],
      [13.5, 14.5, 15.5, 16.5],
    ].into();

    assert!(matrix[0][0].approx_eq(1.0));
    assert!(matrix[0][3].approx_eq(4.0));
    assert!(matrix[1][0].approx_eq(5.5));
    assert!(matrix[1][2].approx_eq(7.5));
    assert!(matrix[2][2].approx_eq(11.0));
    assert!(matrix[3][0].approx_eq(13.5));
    assert!(matrix[3][2].approx_eq(15.5));
  }

  #[test]
  #[allow(unused_variables)]
  fn modify_matrix4x4() {
    let mut matrix: Matrix4x4 = [[0.0; 4]; 4].into();
    matrix[1][3] = 3.14;
    let expected = [
      [0.0; 4],
      [0.0, 0.0, 0.0, 3.14],
      [0.0; 4],
      [0.0; 4]
    ].into();

    assert!(matrix[1][3].approx_eq(3.14));
    assert!(matrix.clone().approx_eq(expected));
    assert!(matches!(matrix, expected));
  }

  #[test]
  fn multiply_matrix4x4() {
    let matrix_a: Matrix4x4 = [
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ].into();
    let matrix_b: Matrix4x4 = [
      [-2.0, 1.0, 2.0,  3.0],
      [ 3.0, 2.0, 1.0, -1.0],
      [ 4.0, 3.0, 6.0,  5.0],
      [ 1.0, 2.0, 7.0,  8.0],
    ].into();
    let result = matrix_a * matrix_b;
    let expected: Matrix4x4 = [
      [20.0, 22.0,  50.0,  48.0],
      [44.0, 54.0, 114.0, 108.0],
      [40.0, 58.0, 110.0, 102.0],
      [16.0, 26.0,  46.0,  42.0],
    ].into();

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn multiply_matrix4x4_with_point() {
    let matrix: Matrix4x4 = [
      [1.0, 2.0, 3.0, 4.0],
      [2.0, 4.0, 4.0, 2.0],
      [8.0, 6.0, 4.0, 1.0],
      [0.0, 0.0, 0.0, 1.0],
    ].into();
    let point: Point = (1.0, 2.0, 3.0).into();
    let result = matrix * point;
    let expected: Point = (18.0, 24.0, 33.0).into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn multiply_matrix4x4_with_vector() {
    let matrix: Matrix4x4 = [
      [1.0, 2.0, 3.0, 4.0],
      [2.0, 4.0, 4.0, 2.0],
      [8.0, 6.0, 4.0, 1.0],
      [0.0, 0.0, 0.0, 1.0],
    ].into();
    let vector: Vector = (1.0, 2.0, 3.0).into();
    let result = matrix * vector;
    let expected: Vector = (14.0, 22.0, 32.0).into(); // no translation
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn multiply_matrix4x4_with_identity() {
    let matrix: Matrix4x4 = [
      [0.0, 1.0,  2.0,  4.0],
      [1.0, 2.0,  4.0,  8.0],
      [2.0, 4.0,  8.0, 16.0],
      [4.0, 8.0, 16.0, 32.0],
    ].into();
    let result = Matrix4x4::IDENTITY * matrix.clone();
    assert!(result.approx_eq(matrix));
  }

  #[test]
  fn multiply_point_with_identity4x4() {
    let point: Point = (1.0, 2.0, 3.0).into();
    let result = Matrix4x4::IDENTITY * point.clone();
    assert!(result.approx_eq(point));
  }

  #[test]
  fn multiply_vector_with_identity4x4() {
    let vector: Vector = (1.0, 2.0, 3.0).into();
    let result = Matrix4x4::IDENTITY * vector.clone();
    assert!(result.approx_eq(vector));
  }

  #[test]
  fn transpose_matrix4x4() {
    let matrix: Matrix4x4 = [
      [0.0, 9.0, 3.0, 0.0],
      [9.0, 8.0, 0.0, 8.0],
      [1.0, 8.0, 5.0, 3.0],
      [0.0, 0.0, 5.0, 8.0],
    ].into();
    let result = matrix.transpose();
    let expected: Matrix4x4 = [
      [0.0, 9.0, 1.0, 0.0],
      [9.0, 8.0, 8.0, 0.0],
      [3.0, 0.0, 5.0, 5.0],
      [0.0, 8.0, 3.0, 8.0],
    ].into();
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn transpose_identity4x4() {
    let result = Matrix4x4::IDENTITY.transpose();
    let expected = Matrix4x4::IDENTITY;
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn determinant_matrix2x2() {
    let matrix: Matrix2x2 = [
      [ 1.0, 5.0],
      [-3.0, 2.0]
    ].into();
    let result = matrix.determinant();
    let expected = 17.0;
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn submatrix_matrix3x3() {
    let matrix: Matrix3x3 = [
      [ 1.0, 5.0,  0.0],
      [-3.0, 2.0,  7.0],
      [ 0.0, 6.0, -3.0]
    ].into();
    let result = matrix.submatrix(0, 2);
    let expected: Matrix2x2 = [
      [-3.0, 2.0],
      [0.0, 6.0]
    ].into();
    assert!(expected.approx_eq(result));
  }

  #[test]
  fn submatrix_matrix4x4() {
    let matrix: Matrix4x4 = [
      [-6.0, 1.0,  1.0, 6.0],
      [-8.0, 5.0,  8.0, 6.0],
      [-1.0, 0.0,  8.0, 2.0],
      [-7.0, 1.0, -1.0, 1.0],
    ].into();
    let result = matrix.submatrix(2, 1);
    let expected: Matrix3x3 = [
      [-6.0,  1.0, 6.0],
      [-8.0,  8.0, 6.0],
      [-7.0, -1.0, 1.0]
    ].into();
    assert!(expected.approx_eq(result));
  }

  #[test]
  fn minor_matrix3x3() {
    let matrix: Matrix3x3 = [
      [3.0,  5.0,  0.0],
      [2.0, -1.0, -7.0],
      [6.0, -1.0,  5.0]
    ].into();
    let minor = matrix.minor(1, 0);
    assert!(minor.approx_eq(25.0));
  }

  #[test]
  fn cofactor_matrix3x3() {
    let matrix: Matrix3x3 = [
      [3.0,  5.0,  0.0],
      [2.0, -1.0, -7.0],
      [6.0, -1.0,  5.0]
    ].into();
    let minor = matrix.minor(0, 0);
    let cofactor = matrix.cofactor(0, 0);
    println!("minor: {}", minor);
    println!("cofac: {}", cofactor);

    assert!(minor.approx_eq(-12.0));
    assert!(cofactor.approx_eq(-12.0));

    let minor = matrix.minor(1, 0);
    let cofactor = matrix.cofactor(1, 0);
    assert!(minor.approx_eq(25.0));
    assert!(cofactor.approx_eq(-25.0));
  }

  #[test]
  fn determinant_matrix3x3() {
    let matrix: Matrix3x3 = [
      [ 1.0, 2.0,  6.0],
      [-5.0, 8.0, -4.0],
      [ 2.0, 6.0,  4.0],
    ].into();
    println!("{}", matrix.cofactor(0, 0));
    println!("{}", matrix.cofactor(0, 1));
    println!("{}", matrix.cofactor(0, 2));
    assert!(matrix.cofactor(0, 0).approx_eq(56.0));
    assert!(matrix.cofactor(0, 1).approx_eq(12.0));
    assert!(matrix.cofactor(0, 2).approx_eq(-46.0));

    let determinant = matrix.determinant();
    assert!(determinant.approx_eq(-196.0));
  }

  #[test]
  fn determinant_matrix4x4() {
    let matrix: Matrix4x4 = [
      [-2.0, -8.0,  3.0,  5.0],
      [-3.0,  1.0,  7.0,  3.0],
      [ 1.0,  2.0, -9.0,  6.0],
      [-6.0,  7.0,  7.0, -9.0],
    ].into();
    assert!(matrix.cofactor(0, 0).approx_eq(690.0));
    assert!(matrix.cofactor(0, 1).approx_eq(447.0));
    assert!(matrix.cofactor(0, 2).approx_eq(210.0));
    assert!(matrix.cofactor(0, 3).approx_eq(51.0));

    let determinant = matrix.determinant();
    assert!(determinant.approx_eq(-4071.0));
  }

  #[test]
  fn invertible_matrix4x4() {
    let matrix: Matrix4x4 = [
      [6.0,  4.0, 4.0,  4.0],
      [5.0,  5.0, 7.0,  6.0],
      [4.0, -9.0, 3.0, -7.0],
      [9.0,  1.0, 7.0, -6.0],
    ].into();
    let determinant = matrix.determinant();
    assert!(determinant.approx_eq(-2120.0));
    assert!(matrix.invertible());
  }

  #[test]
  fn noninvertible_matrix4x4() {
    let matrix: Matrix4x4 = [
      [-4.0,  2.0, -2.0, -3.0],
      [ 9.0,  6.0,  2.0,  6.0],
      [ 0.0, -5.0,  1.0, -5.0],
      [ 0.0,  0.0,  0.0,  0.0],
    ].into();
    let determinant = matrix.determinant();
    assert!(determinant.approx_eq(0.0));
    assert!(!matrix.invertible());
  }

  #[test]
  fn inverse_matrix4x4() {
    let matrix: Matrix4x4 = [
      [-5.0,  2.0,  6.0, -8.0],
      [ 1.0, -5.0,  1.0,  8.0],
      [ 7.0,  7.0, -6.0, -7.0],
      [ 1.0, -3.0,  7.0,  4.0],
    ].into();
    let inverse = matrix.inverse().unwrap();

    assert!(matrix.determinant().approx_eq(532.0));
    assert!(matrix.cofactor(2, 3).approx_eq(-160.0));
    assert!(inverse[3][2].approx_eq(-160.0/532.0));
    assert!(matrix.cofactor(3, 2).approx_eq(105.0));
    assert!(inverse[2][3].approx_eq(105.0/532.0));

    let expected: Matrix4x4 = [
      [ 0.21805,  0.45113,  0.24060, -0.04511],
      [-0.80827, -1.45677, -0.44361,  0.52068],
      [-0.07895, -0.22368, -0.05263,  0.19737],
      [-0.52256, -0.81391, -0.30075,  0.30639],
    ].into();

    assert!(inverse.approx_eq(expected));
  }

  #[test]
  fn inverse_matrix4x4_2() {
    let matrix: Matrix4x4 = [
      [ 8.0, -5.0,  9.0,  2.0],
      [ 7.0,  5.0,  6.0,  1.0],
      [-6.0,  0.0,  9.0,  6.0],
      [-3.0,  0.0, -9.0, -4.0],
    ].into();
    let inverse = matrix.inverse().unwrap();

    let expected: Matrix4x4 = [
      [-0.15385, -0.15385, -0.28205, -0.53846],
      [-0.07692,  0.12308,  0.02564,  0.03077],
      [ 0.35897,  0.35897,  0.43590,  0.92308],
      [-0.69231, -0.69231, -0.76923, -1.92308],
    ].into();

    assert!(inverse.approx_eq(expected));
  }

  #[test]
  fn inverse_matrix4x4_3() {
    let matrix: Matrix4x4 = [
      [ 9.0,  3.0,  0.0,  9.0],
      [-5.0, -2.0, -6.0, -3.0],
      [-4.0,  9.0,  6.0,  4.0],
      [-7.0,  6.0,  6.0,  2.0],
    ].into();
    let inverse = matrix.inverse().unwrap();

    let expected: Matrix4x4 = [
      [-0.04074, -0.07778,  0.14444, -0.22222],
      [-0.07778,  0.03333,  0.36667, -0.33333],
      [-0.02901, -0.14630, -0.10926,  0.12963],
      [ 0.17778,  0.06667, -0.26667,  0.33333],
    ].into();

    assert!(inverse.approx_eq(expected));
  }

  #[test]
  fn product_multiplied_by_its_inverse() {
    let matrix_a: Matrix4x4 = [
      [ 3.0, -9.0,  7.0,  3.0],
      [ 3.0, -8.0,  2.0, -9.0],
      [-4.0,  4.0,  4.0,  1.0],
      [-6.0,  5.0, -1.0,  1.0],
    ].into();
    let matrix_b: Matrix4x4 = [
      [8.0,  2.0, 2.0, 2.0],
      [3.0, -1.0, 7.0, 0.0],
      [7.0,  0.0, 5.0, 4.0],
      [6.0, -2.0, 0.0, 5.0],
    ].into();

    let matrix_c = matrix_a.clone() * matrix_b.clone();
    assert!(matrix_a.approx_eq(matrix_c * matrix_b.inverse().unwrap()));
  }
}
