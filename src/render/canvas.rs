use crate::colour::Colour;
use image::{ImageBuffer, Rgb, RgbImage};
use std::fmt::Write as _;

pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pub canvas: Vec<Colour>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    let canvas = (0..width * height).map(|_| Colour::BLACK).collect();

    Canvas {
      width,
      height,
      canvas,
    }
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
    assert!(x < self.width);
    assert!(y < self.height);

    let pixel = self.canvas.get_mut(y * self.width + x).unwrap();
    *pixel = colour;
  }

  pub fn pixel_at(&self, x: usize, y: usize) -> Colour {
    assert!(x < self.width);
    assert!(y < self.height);

    *self.canvas.get(y * self.width + x).unwrap()
  }

  /// Returns the canvas formatted as a PPM image
  pub fn to_ppm(&self) -> Result<String, std::fmt::Error> {
    const MAX_COLOUR_VALUE: usize = 255;

    let (width, height) = (self.width, self.height);
    let mut string = String::new();
    // Header
    writeln!(string, "P3")?;
    writeln!(string, "{width} {height}")?;
    writeln!(string, "{MAX_COLOUR_VALUE}")?;
    // Body
    for y in 0..height {
      let mut line_start = string.len();
      for x in 0..width {
        let colour = self.pixel_at(x, y);
        for value in [colour.red, colour.green, colour.blue] {
          // cast to the range 0-255
          let value =
            ((value.clamp(0.0, 1.0) * MAX_COLOUR_VALUE as f32).round() as usize).to_string();

          if string.len() + value.len() - line_start >= 70 {
            string.pop(); // remove the trailing space from the previous line
            string.push('\n');
            line_start = string.len();
          }
          write!(string, "{value} ").unwrap();
        }
      }
      string.pop(); // remove the trailing space from the previous line
      string.push('\n');
    }

    Ok(string)
  }

  /// Returns the canvas as an ImageBuffer
  pub fn to_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img: RgbImage = ImageBuffer::new(self.width as _, self.height as _);

    img
      .pixels_mut()
      .zip(&self.canvas)
      .map(|(pixel, colour)| {
        *pixel = (*colour).into();
      })
      .count();

    img
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_canvas() {
    let canvas = Canvas::new(10, 20);
    assert_eq!(canvas.width, 10);
    assert_eq!(canvas.height, 20);
    for colour in canvas.canvas {
      assert_eq!(
        colour,
        Colour {
          red: 0.0,
          green: 0.0,
          blue: 0.0
        }
      )
    }
  }

  #[test]
  fn write_pixel() {
    let mut canvas = Canvas::new(10, 20);
    canvas.write_pixel(2, 3, Colour::RED);
  }

  #[test]
  fn ppm_header() {
    let canvas = Canvas::new(5, 3);
    let ppm = canvas.to_ppm().unwrap();
    let result = ppm.lines().take(3).collect::<Vec<_>>();
    let expected = vec!["P3", "5 3", "255"];
    assert_eq!(result, expected);
  }

  #[test]
  fn ppm_pixel_data() {
    let mut canvas = Canvas::new(5, 3);
    canvas.write_pixel(0, 0, (1.5, 0.0, 0.0).into());
    canvas.write_pixel(2, 1, (0.0, 0.5, 0.0).into());
    canvas.write_pixel(4, 2, (-0.5, 0.0, 1.0).into());

    let ppm = canvas.to_ppm().unwrap();
    let result = ppm.lines().skip(3).take(3).collect::<Vec<_>>();
    let expected = vec![
      "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
      "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
      "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
    ];

    assert_eq!(result, expected);
  }

  #[test]
  fn ppm_split_long_lines() {
    let mut canvas = Canvas::new(10, 2);
    let colour = (1.0, 0.8, 0.6).into();
    for y in 0..2 {
      for x in 0..10 {
        canvas.write_pixel(x, y, colour);
      }
    }
    let ppm = canvas.to_ppm().unwrap();
    let result = ppm.lines().skip(3).take(4).collect::<Vec<_>>();
    let expected = vec![
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
      "153 255 204 153 255 204 153 255 204 153 255 204 153",
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
      "153 255 204 153 255 204 153 255 204 153 255 204 153",
    ];

    assert_eq!(result, expected);
  }

  #[test]
  fn ppm_ends_with_newline() {
    let canvas = Canvas::new(5, 3);
    let ppm = canvas.to_ppm().unwrap();
    let last = ppm.chars().next_back().unwrap();
    assert_eq!(last, '\n');
  }
}
