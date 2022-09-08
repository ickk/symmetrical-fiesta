use crate::*;

pub struct PointLight {
  pub position: Point,
  pub intensity: Colour,
}

impl PointLight {
  pub fn new(position: impl Into<Point>, intensity: impl Into<Colour>) -> Self {
    PointLight {
      position: position.into(),
      intensity: intensity.into(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_point_light() {
    let position = Point::from((0.0, 0.0, 0.0));
    let intensity = Colour::from((1.0, 1.0, 1.0));
    let light = PointLight::new(position, intensity);

    assert!(light.position.approx_eq(position));
    assert_eq!(light.intensity, intensity);
  }
}
