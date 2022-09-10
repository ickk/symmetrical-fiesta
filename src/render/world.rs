use crate::*;

pub struct World {
  pub objects: Vec<Box<dyn Object>>,
  pub lights: Vec<PointLight>,
}

impl World {
  pub fn new() -> Self {
    World {
      objects: vec![],
      lights: vec![],
    }
  }

  pub fn intersect(&self, ray: Ray) -> IntersectionCollection {
    let mut intersections = IntersectionCollection::new();
    self
      .objects
      .iter()
      .fold(&mut intersections, |acc, x| acc.merge(x.intersect(ray)));
    intersections
  }

  pub fn shade_hit(&self, computations: &IntersectionComputations) -> Colour {
    let shadowed = self.is_shadowed(computations.over_position);
    let local_position =
      computations.object.transform().inverse().unwrap() * computations.over_position;

    computations.object.material().lighting(
      &self.lights[0],
      computations.over_position,
      computations.eye,
      computations.normal,
      shadowed,
      local_position,
    )
  }

  pub fn colour_at(&self, ray: Ray) -> Colour {
    let intersections = self.intersect(ray);
    if let Some(hit) = intersections.hit() {
      let computations = hit.prepare_computations(ray);
      self.shade_hit(&computations)
    } else {
      Colour::BLACK
    }
  }

  pub fn is_shadowed(&self, position: Point) -> bool {
    let point_to_light = self.lights[0].position - position;
    let distance = point_to_light.magnitude();
    let ray = Ray::new(position, point_to_light.normalise());

    self
      .intersect(ray)
      .hit()
      .map_or(false, |hit| hit.t < distance)
  }
}

impl Default for World {
  fn default() -> World {
    let mut world = World::new();
    world
      .lights
      .push(PointLight::new((-10.0, 10.0, -10.0), (1.0, 1.0, 1.0)));

    let mut sphere_1 = Sphere::new();
    sphere_1.material = Material {
      pattern: Pattern::solid(Colour::new(0.8, 1.0, 0.6)),
      diffuse: 0.7,
      specular: 0.2,
      ..Default::default()
    };
    let mut sphere_2 = Sphere::new();
    sphere_2.transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    world.objects.push(Box::new(sphere_1));
    world.objects.push(Box::new(sphere_2));

    world
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_world() {
    let world = World::new();

    let n_objects = world.objects.len();
    let n_lights = world.lights.len();

    assert_eq!(n_objects, 0);
    assert_eq!(n_lights, 0);
  }

  #[test]
  fn default_world() {
    let world = World::default();

    let sphere_1 = &world.objects[0];
    assert!(sphere_1.transform().approx_eq(Matrix4x4::IDENTITY));
    assert!(sphere_1.material().diffuse.approx_eq(0.7));
    assert!(sphere_1.material().specular.approx_eq(0.2));
    let _expected = Pattern::solid(Colour::new(0.8, 1.0, 0.6));
    assert!(matches!(&sphere_1.material().pattern, _expected));

    let sphere_2 = &world.objects[1];
    assert!(sphere_2
      .transform()
      .approx_eq(Matrix4x4::scale(0.5, 0.5, 0.5)));

    let light = &world.lights[0];
    assert!(light.approx_eq(PointLight::new((-10.0, 10.0, -10.0), (1.0, 1.0, 1.0))));
  }

  #[test]
  fn intersect_world() {
    let world = World::default();
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let intersections = world.intersect(ray);

    assert_eq!(intersections.len(), 4);
    assert!(intersections[0].t.approx_eq(4.0));
    assert!(intersections[1].t.approx_eq(4.5));
    assert!(intersections[2].t.approx_eq(5.5));
    assert!(intersections[3].t.approx_eq(6.0));
  }

  #[test]
  fn precomputing_intersection_state() {
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let intersection = Intersection {
      t: 4.0,
      object: &shape,
    };

    let comps = intersection.prepare_computations(ray);

    assert!(comps.t.approx_eq(4.0));
    assert!(std::ptr::eq(comps.object, intersection.object));
    assert!(comps.eye.approx_eq(Vector::from((0.0, 0.0, -1.0))));
    assert!(comps.normal.approx_eq(Vector::from((0.0, 0.0, -1.0))));
  }

  #[test]
  fn shading_intersection() {
    let world = World::default();
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let shape = &world.objects[0];
    let intersection = Intersection {
      t: 4.0,
      object: &**shape,
    };
    let computations = &intersection.prepare_computations(ray);

    let colour = world.shade_hit(computations);
    eprintln!("{colour:?}");
    assert!(colour.approx_eq(Colour::from((0.38063, 0.47578, 0.28547))));
  }

  #[test]
  fn shading_intersection_from_inside() {
    let mut world = World::default();
    world.lights[0] = PointLight::new((0.0, 0.25, 0.0), (1.0, 1.0, 1.0));
    let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
    let shape = &world.objects[1];
    let intersection = Intersection {
      t: 0.5,
      object: &**shape,
    };
    let computations = intersection.prepare_computations(ray);
    let colour = world.shade_hit(&computations);
    eprintln!("{colour:?}");
    assert!(colour.approx_eq(Colour::from((0.90450, 0.90450, 0.90450))));
  }

  #[test]
  fn colour_ray_miss() {
    let world = World::default();
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 1.0, 0.0));
    let colour = world.colour_at(ray);
    assert!(colour.approx_eq(Colour::new(0.0, 0.0, 0.0)));
  }

  #[test]
  fn colour_ray_hit() {
    let world = World::default();
    let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let colour = world.colour_at(ray);
    eprintln!("{colour:?}");
    assert!(colour.approx_eq(Colour::new(0.38063, 0.47578, 0.28547)));
  }

  #[test]
  fn colour_with_intersection_behind_ray() {
    let (world, inner_colour) = {
      let mut world = World::new();
      world
        .lights
        .push(PointLight::new((-10.0, 10.0, -10.0), (1.0, 1.0, 1.0)));

      let mut outer = Sphere::new();
      outer.material = Material {
        pattern: Pattern::solid((0.8, 1.0, 0.6).into()),
        diffuse: 0.7,
        specular: 0.2,
        ambient: 1.0,
        ..Default::default()
      };
      let mut inner = Sphere::new();
      inner.transform = Matrix4x4::scale(0.5, 0.5, 0.5);
      let inner_colour = Colour::new(0.2, 0.3, 0.4);
      inner.material = Material {
        ambient: 1.0,
        pattern: Pattern::solid(inner_colour),
        ..Default::default()
      };

      world.objects.push(Box::new(outer));
      world.objects.push(Box::new(inner));
      (world, inner_colour)
    };
    let ray = Ray::new((0.0, 0.0, 0.75), (0.0, 0.0, -1.0));

    let colour = world.colour_at(ray);
    assert!(colour.approx_eq(inner_colour));
  }

  #[test]
  fn no_shadow_when_nothing_is_colinear() {
    let world = World::default();
    let position = Point::new(0.0, 10.0, 0.0);

    assert!(!world.is_shadowed(position));
  }

  #[test]
  fn shadow_when_object_between_point_and_light() {
    let world = World::default();
    let position = Point::new(10.0, -10.0, 10.0);

    assert!(world.is_shadowed(position));
  }

  #[test]
  fn no_shadow_when_object_behind_light() {
    let world = World::default();
    let position = Point::new(-20.0, 20.0, -20.0);

    assert!(!world.is_shadowed(position));
  }

  #[test]
  fn no_shadow_when_object_behind_point() {
    let world = World::default();
    let position = Point::new(-2.0, 2.0, -2.0);

    assert!(!world.is_shadowed(position));
  }

  #[test]
  fn shading_shadow() {
    let mut world = World::new();
    world
      .lights
      .push(PointLight::new((0.0, 0.0, -10.0), (1.0, 1.0, 1.0)));
    let sphere_1 = Box::new(Sphere::new());
    let mut sphere_2 = Box::new(Sphere::new());
    sphere_2.transform = Matrix4x4::translation(0.0, 0.0, 10.0);
    world.objects.push(sphere_1);
    world.objects.push(sphere_2);

    let ray = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
    let intersection = Intersection {
      t: 4.0,
      object: &*world.objects[1],
    };
    let computations = intersection.prepare_computations(ray);

    let colour = world.shade_hit(&computations);
    let expected = Colour::new(0.1, 0.1, 0.1);
    assert!(colour.approx_eq(expected));
  }
}
