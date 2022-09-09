use crate::*;

#[derive(Debug, Clone)]
pub struct Material {
  pub colour: Colour,
  pub ambient: f32,
  pub diffuse: f32,
  pub specular: f32,
  pub shininess: f32,
}

impl Default for Material {
  fn default() -> Material {
    Material {
      colour: Colour::WHITE,
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
    }
  }
}

impl Material {
  pub fn lighting(
    &self,
    light: &PointLight,
    position: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    in_shadow: bool,
  ) -> Colour {
    let effective_colour = self.colour * light.intensity;
    let light_vector = (light.position - position).normalise();
    let ambient = effective_colour * self.ambient;

    let light_dot_normal = light_vector.dot(normal_vector);

    let (diffuse, specular);
    if light_dot_normal < 0.0 || in_shadow {
      diffuse = Colour::BLACK;
      specular = Colour::BLACK;
    } else {
      diffuse = effective_colour * self.diffuse * light_dot_normal;

      let reflect_vector = (-light_vector).reflect(normal_vector);
      let reflect_dot_eye = reflect_vector.dot(eye_vector);
      if reflect_dot_eye <= 0.0 {
        specular = Colour::BLACK;
      } else {
        specular = light.intensity * self.specular * reflect_dot_eye.powf(self.shininess)
      }
    }

    ambient + diffuse + specular
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_default_material() {
    let _default_material = Material::default();
    let _expected = Material {
      colour: Colour::WHITE,
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
    };
    assert!(matches!(_default_material, _expected))
  }

  #[test]
  fn lighting_camera_between_light_and_surface() {
    let eye_vector = Vector::from((0.0, 0.0, -1.0));
    let normal_vector = Vector::from((0.0, 0.0, -1.0));
    let light = PointLight::new((0.0, 0.0, -10.0), (1.0, 1.0, 1.0));
    let material = Material::default();
    let position: Point = (0.0, 0.0, 0.0).into();

    let result = material.lighting(&light, position, eye_vector, normal_vector, false);
    let expected = Colour::from((1.9, 1.9, 1.9));

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn lighting_camera_offset_45_degrees() {
    let eye_vector = Vector::from((0.0, 1.0 / 2.0f32.sqrt(), -1.0 / 2.0f32.sqrt()));
    let normal_vector = Vector::from((0.0, 0.0, -1.0));
    let light = PointLight::new((0.0, 0.0, -10.0), (1.0, 1.0, 1.0));
    let material = Material::default();
    let position: Point = (0.0, 0.0, 0.0).into();

    let result = material.lighting(&light, position, eye_vector, normal_vector, false);
    let expected = Colour::from((1.0, 1.0, 1.0));

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn lighting_light_offset_45_degrees() {
    let eye_vector = Vector::from((0.0, 0.0, -1.0));
    let normal_vector = Vector::from((0.0, 0.0, -1.0));
    let light = PointLight::new((0.0, 10.0, -10.0), (1.0, 1.0, 1.0));
    let material = Material::default();
    let position: Point = (0.0, 0.0, 0.0).into();

    let result = material.lighting(&light, position, eye_vector, normal_vector, false);
    let expected = Colour::from((0.7364, 0.7364, 0.7364));

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn lighting_camera_in_path_of_reflection_vector() {
    let eye_vector = Vector::from((0.0, -1.0 / 2.0f32.sqrt(), -1.0 / 2.0f32.sqrt()));
    let normal_vector = Vector::from((0.0, 0.0, -1.0));
    let light = PointLight::new((0.0, 10.0, -10.0), (1.0, 1.0, 1.0));
    let material = Material::default();
    let position: Point = (0.0, 0.0, 0.0).into();

    let result = material.lighting(&light, position, eye_vector, normal_vector, false);
    let expected = Colour::from((1.6364, 1.6364, 1.6364));

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn lighting_light_behind_surface() {
    let eye_vector = Vector::from((0.0, 0.0, -1.0));
    let normal_vector = Vector::from((0.0, 0.0, -1.0));
    let light = PointLight::new((0.0, 0.0, 10.0), (1.0, 1.0, 1.0));
    let material = Material::default();
    let position: Point = (0.0, 0.0, 0.0).into();

    let result = material.lighting(&light, position, eye_vector, normal_vector, false);
    let expected = Colour::from((0.1, 0.1, 0.1));

    assert!(result.approx_eq(expected));
  }

  #[test]
  fn lighting_surface_in_shadow() {
    let material = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);

    let eye_vector = Vector::new(0.0, 0.0, -1.0);
    let normal_vector = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.00, -10.0), Colour::new(1.0, 1.0, 1.0));

    let result = material.lighting(&light, position, eye_vector, normal_vector, true);
    let expected = Colour::new(0.1, 0.1, 0.1);

    assert!(result.approx_eq(expected));
  }
}
