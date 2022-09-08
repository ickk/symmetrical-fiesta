pub mod sphere;

pub use sphere::*;

use crate::math::*;

use itertools::Itertools;
use std::{fmt::Debug, ops::Index};

pub trait Object: Debug {
  fn intersect(&self, ray: Ray) -> IntersectionCollection;
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
  pub t: f32,
  pub object: &'a dyn Object,
}

// Could this be a sorted heap?
pub struct IntersectionCollection<'a> {
  // This list always remains sorted from smallest to largest, based on the t values
  inner: Vec<Intersection<'a>>,
}

impl<'a> IntersectionCollection<'a> {
  pub fn new() -> Self {
    IntersectionCollection { inner: vec![] }
  }

  // Trusts that vec is sorted in ascending order
  pub fn from_vec_unchecked(vec: Vec<Intersection<'a>>) -> Self {
    IntersectionCollection { inner: vec }
  }

  pub fn len(&self) -> usize {
    self.inner.len()
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  // this could be a binary search
  pub fn hit(&self) -> Option<&Intersection> {
    for intersection in &self.inner {
      if intersection.t >= 0.0 {
        return Some(intersection);
      }
    }
    None
  }

  // inserts an intersection into the collection while maintaining the sorted order
  pub fn insert(&mut self, intersection: Intersection<'a>) {
    self.inner.push(intersection);
    for (i, prev_i) in (0..self.inner.len()).rev().tuple_windows() {
      if self.inner[i].t >= self.inner[prev_i].t {
        break;
      }
      self.inner.swap(i, prev_i);
    }
  }
}

impl<'a> Index<usize> for IntersectionCollection<'a> {
  type Output = Intersection<'a>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.inner[index]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hit_all_positive_ts() {
    let sphere = Sphere::new();
    let intersection_a = Intersection {
      t: 1.0,
      object: &sphere,
    };
    let intersection_b = Intersection {
      t: 2.0,
      object: &sphere,
    };

    let intersections =
      IntersectionCollection::from_vec_unchecked(vec![intersection_a, intersection_b]);
    let hit = intersections.hit().unwrap();

    assert!(std::ptr::eq(hit, &intersections.inner[0]));
  }

  #[test]
  fn hit_some_negative_ts() {
    let sphere = Sphere::new();
    let intersection_a = Intersection {
      t: -1.0,
      object: &sphere,
    };
    let intersection_b = Intersection {
      t: 1.0,
      object: &sphere,
    };

    let intersections =
      IntersectionCollection::from_vec_unchecked(vec![intersection_a, intersection_b]);
    let hit = intersections.hit().unwrap();

    assert!(std::ptr::eq(hit, &intersections.inner[1]));
  }

  #[test]
  fn hit_all_negative_ts() {
    let sphere = Sphere::new();
    let intersection_a = Intersection {
      t: -2.0,
      object: &sphere,
    };
    let intersection_b = Intersection {
      t: -1.0,
      object: &sphere,
    };

    let intersections =
      IntersectionCollection::from_vec_unchecked(vec![intersection_a, intersection_b]);
    let hit = intersections.hit();

    assert!(hit.is_none());
  }

  #[test]
  #[allow(unused_variables)]
  fn hit_is_lowest_nonnegative() {
    let sphere = Sphere::new();

    let intersection_a = Intersection {
      t: 5.0,
      object: &sphere,
    };
    let intersection_b = Intersection {
      t: 7.0,
      object: &sphere,
    };
    let intersection_c = Intersection {
      t: -3.0,
      object: &sphere,
    };
    let intersection_d = Intersection {
      t: 2.0,
      object: &sphere,
    };

    let mut intersections = IntersectionCollection::new();
    [
      intersection_a,
      intersection_b,
      intersection_c,
      intersection_d.clone(),
    ]
    .map(|i| intersections.insert(i));

    let hit = intersections.hit().unwrap();
    assert!(matches!(hit, intersection_d));
  }
}
