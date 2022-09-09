use crate::math::*;
use image::Rgb;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}

impl From<Colour> for Rgb<u8> {
  fn from(colour: Colour) -> Rgb<u8> {
    [
      (colour.red.clamp(0.0, 1.0) * 255.0).round() as u8,
      (colour.green.clamp(0.0, 1.0) * 255.0).round() as u8,
      (colour.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
    .into()
  }
}

impl ApproxEq for Colour {
  fn approx_eq(&self, rhs: Colour) -> bool {
    self.red.approx_eq(rhs.red) && self.green.approx_eq(rhs.green) && self.blue.approx_eq(rhs.blue)
  }
}

impl From<Tuple3> for Colour {
  fn from(tuple: Tuple3) -> Colour {
    Colour {
      red: tuple.0,
      green: tuple.1,
      blue: tuple.2,
    }
  }
}

impl Add for Colour {
  type Output = Colour;

  fn add(self, rhs: Colour) -> Colour {
    Colour {
      red: self.red + rhs.red,
      green: self.green + rhs.green,
      blue: self.blue + rhs.blue,
    }
  }
}

impl Sub for Colour {
  type Output = Colour;

  fn sub(self, rhs: Colour) -> Colour {
    Colour {
      red: self.red - rhs.red,
      green: self.green - rhs.green,
      blue: self.blue - rhs.blue,
    }
  }
}

impl Mul<f32> for Colour {
  type Output = Colour;

  fn mul(self, rhs: f32) -> Colour {
    Colour {
      red: self.red * rhs,
      green: self.green * rhs,
      blue: self.blue * rhs,
    }
  }
}

impl Mul<Colour> for f32 {
  type Output = Colour;

  fn mul(self, rhs: Colour) -> Colour {
    Colour {
      red: self * rhs.red,
      green: self * rhs.green,
      blue: self * rhs.blue,
    }
  }
}

impl Mul<Colour> for Colour {
  type Output = Colour;

  fn mul(self, rhs: Colour) -> Colour {
    self.hadamard_product(rhs)
  }
}

impl Colour {
  pub const BLACK: Colour = Colour {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
  };
  pub const RED: Colour = Colour {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
  };
  pub const GREEN: Colour = Colour {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
  };
  pub const BLUE: Colour = Colour {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
  };
  pub const WHITE: Colour = Colour {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
  };

  pub fn new(red: f32, green: f32, blue: f32) -> Self {
    Colour { red, green, blue }
  }

  pub fn hadamard_product(self, rhs: Colour) -> Colour {
    Colour {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::math::ApproxEq;

  #[test]
  fn construct_colour() {
    // construct Colour from tuple
    let colour: Colour = (-0.5, 0.4, 1.7).into();
    let _expected = Colour {
      red: -0.5,
      green: 0.4,
      blue: 1.7,
    };
    assert!(matches!(colour, _expected));
  }

  #[test]
  fn add_colours() {
    let colour_a: Colour = (0.9, 0.6, 0.75).into();
    let colour_b: Colour = (0.7, 0.1, 0.25).into();
    let result = colour_a + colour_b;
    let expected = Colour {
      red: 1.6,
      green: 0.7,
      blue: 1.0,
    };
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn subtract_colours() {
    let colour_a: Colour = (0.9, 0.6, 0.75).into();
    let colour_b: Colour = (0.7, 0.1, 0.25).into();
    let result = colour_a - colour_b;
    let expected = Colour {
      red: 0.2,
      green: 0.5,
      blue: 0.5,
    };
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn scalar_multiplication_colour() {
    // scalar on the right
    let colour: Colour = (0.2, 0.3, 0.4).into();
    let result = colour * 2.0;
    let expected = Colour {
      red: 0.4,
      green: 0.6,
      blue: 0.8,
    };
    assert!(result.approx_eq(expected));

    // scalar on the left
    let result = 2.0 * colour;
    assert!(result.approx_eq(expected));
  }

  #[test]
  fn hadamard_product_colour() {
    let colour_a: Colour = (1.0, 0.2, 0.4).into();
    let colour_b: Colour = (0.9, 1.0, 0.1).into();
    let result = colour_a.hadamard_product(colour_b);
    let expected = Colour {
      red: 0.9,
      green: 0.2,
      blue: 0.04,
    };
    assert!(result.approx_eq(expected));

    let result = colour_a * colour_b;
    assert!(result.approx_eq(expected));
  }
}
