use crate::*;
use itertools::Itertools;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
  pub t: f32,
  pub object: &'a dyn Object,
}

#[derive(Debug)]
pub struct IntersectionComputations<'a> {
  pub t: f32,
  pub object: &'a dyn Object,
  pub position: Point,
  pub over_position: Point,
  pub eye: Vector,
  pub normal: Vector,
  pub kind: IntersectionType,
  pub reflect: Vector,
}

#[derive(Debug)]
pub enum IntersectionType {
  Inside,
  Outside,
}

impl Intersection<'_> {
  pub fn prepare_computations(&self, ray: Ray) -> IntersectionComputations {
    use IntersectionType::*;

    let position = ray.position(self.t);
    let eye = -ray.direction;
    let mut normal = self.object.normal_at(position);
    let kind;
    if normal.dot(eye) < 0.0 {
      kind = Inside;
      normal = -normal;
    } else {
      kind = Outside;
    }
    let over_position = position + normal * 0.0015;
    let reflect = ray.direction.reflect(normal);

    IntersectionComputations {
      t: self.t,
      object: self.object,
      position,
      over_position,
      eye,
      normal,
      kind,
      reflect,
    }
  }
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

  /// Trusts that vec is sorted in ascending order
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

  /// Inserts an intersection into the collection while maintaining the sorted order
  pub fn insert(&mut self, intersection: Intersection<'a>) {
    self.inner.push(intersection);
    for (i, prev_i) in (0..self.inner.len()).rev().tuple_windows() {
      if self.inner[i].t >= self.inner[prev_i].t {
        break;
      }
      self.inner.swap(i, prev_i);
    }
  }

  // TODO: this could be optimised
  pub fn merge(&mut self, rhs: Self) -> &mut Self {
    for intersection in rhs.inner {
      self.insert(intersection)
    }
    self
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
    let _intersection_d = Intersection {
      t: 2.0,
      object: &sphere,
    };

    let mut intersections = IntersectionCollection::new();
    [
      intersection_a,
      intersection_b,
      intersection_c,
      _intersection_d.clone(),
    ]
    .map(|i| intersections.insert(i));

    let _hit = intersections.hit().unwrap();
    assert!(matches!(_hit, _intersection_d));
  }

  #[test]
  fn external_intersection() {
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let intersection = Intersection {
      t: 4.0,
      object: &shape,
    };
    let _computations = intersection.prepare_computations(ray);
    assert!(matches!(_computations.kind, IntersectionType::Outside));
  }

  #[test]
  fn internal_intersection() {
    let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let intersection = Intersection {
      t: 1.0,
      object: &shape,
    };
    let _computations = intersection.prepare_computations(ray);
    assert!(matches!(_computations.kind, IntersectionType::Inside));
    assert!(_computations.eye.approx_eq(Vector::from((0.0, 0.0, -1.0))));
    assert!(_computations
      .position
      .approx_eq(Point::from((0.0, 0.0, 1.0))));
    assert!(_computations
      .normal
      .approx_eq(Vector::from((0.0, 0.0, -1.0))));
  }

  #[test]
  fn hit_over_position_property() {
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = Matrix4x4::translation(0.0, 0.0, 1.0);
    let intersection = Intersection {
      t: 5.0,
      object: &sphere,
    };
    let computations = intersection.prepare_computations(ray);
    assert!(computations.over_position.z < -EPSILON / 2.0);
    assert!(computations.position.z > computations.over_position.z);
  }

  #[test]
  fn prepare_computations_reflection_vector() {
    let shape = Plane::new();
    let ray = Ray::new(
      (0.0, 1.0, -1.0),
      (0.0, -1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt()),
    );
    let intersection = Intersection {
      t: 2.0f32.sqrt(),
      object: &shape,
    };
    let reflect = intersection.prepare_computations(ray).reflect;

    let expected = Vector::new(0.0, 1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt());
    assert!(reflect.approx_eq(expected));
  }
}
