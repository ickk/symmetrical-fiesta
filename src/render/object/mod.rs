pub mod intersection;
pub mod sphere;
pub use intersection::*;
pub use sphere::*;

use crate::*;
use std::fmt::Debug;

pub trait Object: Debug {
  fn intersect(&self, ray: Ray) -> IntersectionCollection;

  fn normal_at(&self, point: Point) -> Vector;

  fn material(&self) -> &Material;

  fn transform(&self) -> &Matrix4x4;
}
