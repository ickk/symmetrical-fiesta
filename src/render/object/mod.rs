pub mod intersection;
pub mod sphere;
pub mod plane;
pub use intersection::*;
pub use sphere::*;
pub use plane::*;

use crate::*;
use std::fmt::Debug;

pub trait Object: Debug {
  fn intersect(&self, ray: Ray) -> IntersectionCollection {
    self.local_intersect(self.transform().inverse().unwrap() * ray)
  }

  fn normal_at(&self, point: Point) -> Vector {
    let inverse_transform = &self.transform().inverse().unwrap();
    let object_normal = self.local_normal_at(inverse_transform * point);
    let world_normal = inverse_transform
      .transpose()
      .mul_vec_unchecked(object_normal);
    world_normal.normalise()
  }

  fn local_intersect(&self, ray: Ray) -> IntersectionCollection;

  fn local_normal_at(&self, point: Point) -> Vector;

  fn material(&self) -> &Material;

  fn transform(&self) -> &Matrix4x4;
}
