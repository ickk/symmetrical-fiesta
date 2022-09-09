use crate::*;

#[derive(Debug)]
pub struct Sphere {
  pub transform: Matrix4x4,
  pub material: Material,
}

impl Sphere {
  pub fn new() -> Self {
    Sphere {
      transform: Matrix4x4::IDENTITY,
      material: Material::default(),
    }
  }
}

impl Object for Sphere {
  /// Returns the t values of the ray where it instersects with the Sphere
  fn intersect(&self, ray: Ray) -> IntersectionCollection {
    let ray = self.transform.inverse().unwrap() * ray;

    let sphere_to_ray = ray.origin - Point::ORIGIN;

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
      IntersectionCollection::from_vec_unchecked(vec![])
    } else {
      IntersectionCollection::from_vec_unchecked(vec![
        Intersection {
          t: -(discriminant.sqrt() + b) / (2.0 * a),
          object: self,
        },
        Intersection {
          t: (discriminant.sqrt() - b) / (2.0 * a),
          object: self,
        },
      ])
    }
  }

  /// Returns the normal vector of the surface sphere at the given point
  fn normal_at(&self, point: Point) -> Vector {
    let inverse_transform = &self.transform.inverse().unwrap();
    let object_point = inverse_transform * point;
    let object_normal = object_point - Point::ORIGIN;
    let world_normal = inverse_transform
      .transpose()
      .mul_vec_unchecked(object_normal);
    world_normal.normalise()
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
  use std::f32::consts::PI;

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

  #[test]
  fn normal_on_x_axis() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(Point::from((1.0, 0.0, 0.0)));
    let expected = Vector::from((1.0, 0.0, 0.0));
    assert!(normal.approx_eq(expected));
  }

  #[test]
  fn normal_on_y_axis() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(Point::from((0.0, 1.0, 0.0)));
    let expected = Vector::from((0.0, 1.0, 0.0));
    assert!(normal.approx_eq(expected));
  }

  #[test]
  fn normal_on_z_axis() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(Point::from((0.0, 0.0, 1.0)));
    let expected = Vector::from((0.0, 0.0, 1.0));
    assert!(normal.approx_eq(expected));
  }

  #[test]
  fn normal_nonaxial() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(Point::from((
      1.0 / 3.0f32.sqrt(),
      1.0 / 3.0f32.sqrt(),
      1.0 / 3.0f32.sqrt(),
    )));
    let expected = Vector::from((
      1.0 / 3.0f32.sqrt(),
      1.0 / 3.0f32.sqrt(),
      1.0 / 3.0f32.sqrt(),
    ));
    assert!(normal.approx_eq(expected));
  }

  #[test]
  fn normal_is_unit_length() {
    let sphere = Sphere::new();
    let normal = sphere.normal_at(Point::from((
      1.0 / 3.0f32.sqrt(),
      1.0 / 3.0f32.sqrt(),
      1.0 / 3.0f32.sqrt(),
    )));
    let expected = normal.normalise();
    assert!(normal.approx_eq(expected));
  }

  #[test]
  fn normal_translated_sphere() {
    let mut sphere = Sphere::new();
    sphere.transform = Matrix4x4::translation(0.0, 1.0, 0.0);

    let normal = sphere.normal_at(Point::from((0.0, 1.70711, -0.70711)));
    let expected = Vector::from((0.0, 0.70711, -0.70711));
    assert!(normal.approx_eq(expected));
  }

  #[test]
  fn normal_transform_sphere() {
    let mut sphere = Sphere::new();
    sphere.transform = Matrix4x4::scale(1.0, 0.5, 1.0) * Matrix4x4::rotation_z(PI / 5.0);

    let normal = sphere.normal_at(Point::from((
      0.0,
      1.0 / 2.0f32.sqrt(),
      -1.0 / 2.0f32.sqrt(),
    )));
    let expected = Vector::from((0.0, 0.97014, -0.24254));
    assert!(normal.approx_eq(expected));
  }

  #[test]
  #[allow(unused_variables)]
  fn sphere_default_material() {
    let sphere = Sphere::new();
    let expected = Material::default();
    assert!(matches!(sphere.material, expected));
  }

  #[test]
  #[allow(unused_variables)]
  fn assign_material_to_sphere() {
    let mut sphere = Sphere::new();
    let material = Material {
      ambient: 1.0,
      ..Default::default()
    };
    sphere.material = material;

    let expected = Material {
      colour: Colour::WHITE,
      ambient: 1.0,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
    };
    assert!(matches!(sphere.material, expected));
  }
}
