use crate::*;

/// A plane spanning X-Z
#[derive(Debug)]
pub struct Plane {
  pub material: Material,
  pub transform: Matrix4x4,
}

impl Plane {
  pub fn new() -> Self {
    Plane {
      material: Material::default(),
      transform: Matrix4x4::IDENTITY,
    }
  }
}

impl Object for Plane {
  fn local_intersect(&self, ray: Ray) -> IntersectionCollection {
    if !ray.direction.y.approx_eq(0.0) {
      return IntersectionCollection::from_vec_unchecked(vec![Intersection {
        t: -ray.origin.y / ray.direction.y,
        object: self,
      }]);
    }
    IntersectionCollection::new()
  }

  fn local_normal_at(&self, _: Point) -> Vector {
    Vector::new(0.0, 1.0, 0.0)
  }

  fn material(&self) -> &Material {
    &self.material
  }

  fn transform(&self) -> &Matrix4x4 {
    &self.transform
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn plane_normal_is_constant_everywhere() {
    let plane = Plane::new();
    let normal_1 = plane.local_normal_at(Point::ORIGIN);
    let normal_2 = plane.local_normal_at(Point::new(10.0, 0.0, -10.0));
    let normal_3 = plane.local_normal_at(Point::new(-5.0, 0.0, 150.0));

    let expected = Vector::new(0.0, 1.0, 0.0);
    assert!(normal_1.approx_eq(expected));
    assert!(normal_2.approx_eq(expected));
    assert!(normal_3.approx_eq(expected));
  }

  #[test]
  fn plane_intersect_with_parallel_ray() {
    let plane = Plane::new();
    let ray = Ray::new((0.0, 10.0, 0.0), (0.0, 0.0, 1.0));
    let intersections = plane.local_intersect(ray);

    assert!(intersections.is_empty());
  }

  #[test]
  fn plane_intersect_with_coplanar_ray() {
    let plane = Plane::new();
    let ray = Ray::new(Point::ORIGIN, (0.0, 0.0, 1.0));
    let intersections = plane.local_intersect(ray);

    assert!(intersections.is_empty());
  }

  #[test]
  fn plane_intersect_with_ray_from_above() {
    let plane = &Plane::new();
    let ray = Ray::new((0.0, 1.0, 0.0), (0.0, -1.0, 0.0));
    let intersections = plane.local_intersect(ray);
    assert_eq!(intersections.len(), 1);
    assert!(intersections[0].t.approx_eq(1.0));
  }

  #[test]
  fn plane_intersect_with_ray_from_below() {
    let plane = Plane::new();
    let ray = Ray::new((0.0, -1.0, 0.0), (0.0, 1.0, 0.0));
    let intersections = plane.local_intersect(ray);
    assert_eq!(intersections.len(), 1);
    assert!(intersections[0].t.approx_eq(1.0));
  }
}
