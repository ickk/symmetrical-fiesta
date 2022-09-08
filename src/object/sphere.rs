use super::*;

#[derive(Debug)]
pub struct Sphere {
  pub transform: Matrix4x4,
}

impl Sphere {
  pub fn new() -> Self {
    Sphere {
      transform: Matrix4x4::IDENTITY,
    }
  }
}

impl Object for Sphere {
  /// Returns the t values of the ray where it instersects with the Sphere
  fn intersect(&self, ray: Ray) -> Vec<Intersection> {
    let ray = self.transform.inverse().unwrap() * ray;

    let sphere_to_ray = ray.origin - Point::ORIGIN;

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
      vec![]
    } else {
      vec![
        Intersection {
          t: -(discriminant.sqrt() + b) / (2.0 * a),
          object: self,
        },
        Intersection {
          t: (discriminant.sqrt() - b) / (2.0 * a),
          object: self,
        },
      ]
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ray_sphere_intersection() {
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let sphere = Sphere::new();

    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert!(xs[0].t.approx_eq(4.0));
    assert!(xs[1].t.approx_eq(6.0));
  }

  #[test]
  fn ray_sphere_intersection_at_tangent() {
    let ray = Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
    let sphere = Sphere::new();

    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert!(xs[0].t.approx_eq(5.0));
    assert!(xs[1].t.approx_eq(5.0));
  }

  #[test]
  fn ray_sphere_intersection_miss() {
    let ray = Ray::new((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
    let sphere = Sphere::new();

    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 0);
  }

  #[test]
  fn ray_sphere_intersection_from_centre() {
    let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
    let sphere = Sphere::new();

    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert!(xs[0].t.approx_eq(-1.0));
    assert!(xs[1].t.approx_eq(1.0));
  }

  #[test]
  fn ray_sphere_intersection_behind() {
    let ray = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
    let sphere = Sphere::new();

    let xs = sphere.intersect(ray);
    assert_eq!(xs.len(), 2);
    assert!(xs[0].t.approx_eq(-6.0));
    assert!(xs[1].t.approx_eq(-4.0));
  }

  #[test]
  #[allow(unused_variables)]
  fn sphere_default_transform() {
    let sphere = Sphere::new();
    let expected = Matrix4x4::IDENTITY;
    assert!(matches!(sphere.transform, expected));
  }

  #[test]
  #[allow(unused_variables)]
  fn sphere_modify_transform() {
    let mut sphere = Sphere::new();
    let transform = Matrix4x4::translation(2.0, 3.0, 4.0);

    sphere.transform = transform.clone();
    assert!(matches!(sphere.transform, transform));
  }

  #[test]
  fn intersect_scaled_sphere() {
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = Matrix4x4::scale(2.0, 2.0, 2.0);
    let xs = sphere.intersect(ray);

    assert_eq!(xs.len(), 2);
    assert!(xs[0].t.approx_eq(3.0));
    assert!(xs[1].t.approx_eq(7.0));
  }

  #[test]
  fn intersect_translated_sphere() {
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = Matrix4x4::translation(5.0, 0.0, 0.0);
    let xs = sphere.intersect(ray);

    assert_eq!(xs.len(), 0);
  }
}
