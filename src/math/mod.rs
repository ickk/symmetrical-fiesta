pub mod matrix;
pub mod point;
pub mod transformation;
pub mod vector;

pub use matrix::*;
pub use point::*;
pub use transformation::*;
pub use vector::*;

pub type Tuple3 = (f32, f32, f32);
pub type Array2 = [f32; 2];
pub type Array2x2 = [Array2; 2];
pub type Array3 = [f32; 3];
pub type Array3x3 = [Array3; 3];
pub type Array4 = [f32; 4];
pub type Array4x4 = [Array4; 4];

pub trait ApproxEq {
  fn approx_eq(&self, rhs: Self) -> bool;
}

pub const EPSILON: f32 = 0.00001;
impl ApproxEq for f32 {
  fn approx_eq(&self, rhs: f32) -> bool {
    (self - rhs).abs() < EPSILON
  }
}
