pub mod point;
pub mod vector;

pub use point::*;
pub use vector::*;

pub type Tuple3 = (f32, f32, f32);

pub trait ApproxEq {
  fn approx_eq(self, rhs: Self) -> bool;
}

pub const EPSILON: f32 = 0.00001;
impl ApproxEq for f32 {
  fn approx_eq(self, rhs: f32) -> bool {
    (self - rhs).abs() < EPSILON
  }
}
