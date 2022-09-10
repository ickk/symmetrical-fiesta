use crate::*;
use perlin2d::PerlinNoise2D;

#[derive(Debug, Clone)]
pub struct Pattern {
  transform: Matrix4x4,
  pattern: PatternType,
}

impl Default for Pattern {
  fn default() -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Solid(Colour::WHITE),
    }
  }
}

impl Pattern {
  #[inline]
  pub fn colour_at(&self, position: Point) -> Colour {
    self
      .pattern
      .local_colour_at(self.transform.inverse().unwrap() * position)
  }

  pub const fn solid(colour: Colour) -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Solid(colour),
    }
  }

  pub const fn test_pattern() -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Test,
    }
  }

  pub const fn stripe(colour_a: Colour, colour_b: Colour) -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Stripe(colour_a, colour_b),
    }
  }

  pub const fn gradient(colour_a: Colour, colour_b: Colour) -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Gradient(colour_a, colour_b),
    }
  }

  pub const fn ring(colour_a: Colour, colour_b: Colour) -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Ring(colour_a, colour_b),
    }
  }

  pub const fn checkers(colour_a: Colour, colour_b: Colour) -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Checkers(colour_a, colour_b),
    }
  }

  pub fn perlin(perlin_obj: PerlinNoise2D, factor: f32, pattern: PatternType) -> Self {
    Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: Perlin(perlin_obj, factor, Box::new(pattern)),
    }
  }

  pub fn with_transform(mut self, transform: Matrix4x4) -> Self {
    self.transform = transform;
    self
  }
}

pub enum PatternType {
  Solid(Colour),
  Test,
  Stripe(Colour, Colour),
  Gradient(Colour, Colour),
  Ring(Colour, Colour),
  Checkers(Colour, Colour),
  Perlin(PerlinNoise2D, f32, Box<PatternType>),
}

impl Clone for PatternType {
  fn clone(&self) -> PatternType {
    match self {
      Perlin(perlin, factor, pattern) => PatternType::Perlin(
        PerlinNoise2D::new(
          perlin.get_octaves(),
          perlin.get_amplitude(),
          perlin.get_frequency(),
          perlin.get_persistence(),
          perlin.get_lacunarity(),
          perlin.get_scale(),
          perlin.get_bias(),
          perlin.get_seed(),
        ),
        *factor,
        pattern.clone(),
      ),
      Solid(c) => Solid(*c),
      Test => Test,
      Stripe(a, b) => Stripe(*a, *b),
      Gradient(a, b) => Gradient(*a, *b),
      Ring(a, b) => Ring(*a, *b),
      Checkers(a, b) => Checkers(*a, *b),
    }
  }
}

impl std::fmt::Debug for PatternType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("PatternType").finish()
  }
}

use PatternType::*;
impl PatternType {
  #[inline]
  pub fn local_colour_at(&self, position: Point) -> Colour {
    let p = position;
    match self {
      Solid(c) => *c,
      Test => Colour {
        red: p.x,
        green: p.y,
        blue: p.z,
      },
      Stripe(a, b) => {
        if p.x.floor() % 2.0 == 0.0 {
          *a
        } else {
          *b
        }
      }
      Gradient(a, b) => {
        if p.x.floor() % 2.0 == 0.0 {
          *a + (*b - *a) * (p.x - p.x.floor())
        } else {
          *b + (*a - *b) * (p.x - p.x.floor())
        }
      }
      Ring(a, b) => {
        if (p.x * p.x + p.z * p.z).sqrt().floor() % 2.0 == 0.0 {
          *a
        } else {
          *b
        }
      }
      Checkers(a, b) => {
        if (p.x.floor() + p.y.floor() + p.z.floor()) % 2.0 == 0.0 {
          *a
        } else {
          *b
        }
      }
      Perlin(perlin_obj, factor, pattern) => {
        let perturbation_x = perlin_obj.get_noise(p.x as f64, p.y as f64) as f32 * factor;
        let perturbation_y = perlin_obj.get_noise(p.y as f64, p.z as f64) as f32 * factor;
        let perturbation_z = perlin_obj.get_noise(p.z as f64, p.x as f64) as f32 * factor;
        pattern.local_colour_at(Point::new(
          p.x + perturbation_x,
          p.y + perturbation_y,
          p.z + perturbation_z,
        ))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_pattern() {
    let pattern = Pattern::default();
    assert!(pattern.transform.approx_eq(Matrix4x4::IDENTITY));
    let _expected = PatternType::Solid(Colour::WHITE);
    assert!(matches!(pattern.pattern, _expected));
  }

  #[test]
  fn pattern_transform_assignment() {
    let pattern = Pattern::default().with_transform(Matrix4x4::translation(1.0, 2.0, 3.0));
    assert!(pattern
      .transform
      .approx_eq(Matrix4x4::translation(1.0, 2.0, 3.0)));
  }

  #[test]
  fn pattern_with_transform() {
    let pattern = Pattern::test_pattern().with_transform(Matrix4x4::scale(2.0, 2.0, 2.0));
    let result = pattern.colour_at(Point::new(2.0, 3.0, 4.0));
    let expected = Colour::new(1.0, 1.5, 2.0);
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn construct_stripe_pattern() {
    let pattern = Pattern::stripe(Colour::WHITE, Colour::BLACK);
    let _expected = Pattern {
      transform: Matrix4x4::IDENTITY,
      pattern: PatternType::Stripe(Colour::WHITE, Colour::BLACK),
    };
    assert!(matches!(pattern, _expected));
  }

  #[test]
  fn stripe_pattern_constant_in_y() {
    let pattern = Pattern::stripe(Colour::WHITE, Colour::BLACK);
    let _expected = Colour::WHITE;
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.0)),
      _expected
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 1.0, 0.0)),
      _expected
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 2.0, 0.0)),
      _expected
    ));
  }

  #[test]
  fn stripe_pattern_constant_in_z() {
    let pattern = Pattern::stripe(Colour::WHITE, Colour::BLACK);
    let _expected = Colour::WHITE;
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.0)),
      _expected
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 1.0)),
      _expected
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 2.0)),
      _expected
    ));
  }

  #[test]
  fn stripe_pattern_alternates_in_x() {
    let pattern = Pattern::stripe(Colour::WHITE, Colour::BLACK);
    let _white = Colour::WHITE;
    let _black = Colour::BLACK;
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.0)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.9, 0.0, 0.0)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(1.0, 0.0, 0.0)),
      _black
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(-0.1, 0.0, 0.0)),
      _black
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(-1.0, 0.0, 0.0)),
      _black
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(-1.1, 0.0, 0.0)),
      _white
    ));
  }

  #[test]
  fn gradient_pattern() {
    let pattern = Pattern::gradient(Colour::WHITE, Colour::BLACK);
    let _expected = Colour {
      red: 1.0,
      green: 1.0,
      blue: 1.0,
    };
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.0)),
      _expected
    ));
    let _expected = Colour {
      red: 0.75,
      green: 0.75,
      blue: 0.75,
    };
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.25, 0.0, 0.0)),
      _expected
    ));
    let _expected = Colour {
      red: 0.5,
      green: 0.5,
      blue: 0.5,
    };
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.5, 0.0, 0.0)),
      _expected
    ));
    let _expected = Colour {
      red: 0.25,
      green: 0.25,
      blue: 0.25,
    };
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.75, 0.0, 0.0)),
      _expected
    ));
  }

  #[test]
  fn ring_pattern() {
    let pattern = Pattern::gradient(Colour::WHITE, Colour::BLACK);
    let _white = Colour::WHITE;
    let _black = Colour::BLACK;
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.0)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(1.0, 0.0, 0.0)),
      _black
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 1.0)),
      _black
    ));
    assert!(matches!(
      pattern
        .pattern
        .local_colour_at(Point::new(0.708, 0.0, 0.708)),
      _black
    ));
  }

  #[test]
  fn checkers_pattern() {
    let pattern = Pattern::checkers(Colour::WHITE, Colour::BLACK);
    let _white = Colour::WHITE;
    let _black = Colour::BLACK;
    // repeat in x
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.0)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.99, 0.0, 0.0)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(1.01, 0.0, 0.0)),
      _black
    ));
    // repeat in y
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.99, 0.0)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 1.01, 0.0)),
      _black
    ));
    // repeat in z
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 0.99)),
      _white
    ));
    assert!(matches!(
      pattern.pattern.local_colour_at(Point::new(0.0, 0.0, 1.01)),
      _black
    ));
  }
}
