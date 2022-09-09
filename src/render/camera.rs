use crate::*;

pub struct Camera {
  pub canvas_width: usize,
  pub canvas_height: usize,
  pub fov: f32,
  transform: Matrix4x4,
  inverse_transform: Matrix4x4,
  pixel_size: f32,
  half_width: f32,
  half_height: f32,
}

impl Camera {
  pub fn new(canvas_width: usize, canvas_height: usize, fov: f32) -> Self {
    let half_view = (fov / 2.0).tan();
    let aspect_ratio = canvas_width as f32 / canvas_height as f32;
    let (half_width, half_height) = if aspect_ratio >= 1.0 {
      (half_view, half_view / aspect_ratio)
    } else {
      (half_view * aspect_ratio, half_view)
    };
    let pixel_size = half_width * 2.0 / canvas_width as f32;

    Camera {
      canvas_width,
      canvas_height,
      fov,
      transform: Matrix4x4::IDENTITY,
      inverse_transform: Matrix4x4::IDENTITY,
      pixel_size,
      half_width,
      half_height,
    }
  }

  // TODO: avoid calculating inverse transform for every ray
  pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
    let x_offset = (x as f32 + 0.5) * self.pixel_size;
    let y_offset = (y as f32 + 0.5) * self.pixel_size;

    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    let inverse_transform = &self.transform.inverse().unwrap();
    let pixel = inverse_transform * Point::new(world_x, world_y, -1.0);
    let origin = inverse_transform * Point::ORIGIN;
    let direction = (pixel - origin).normalise();

    Ray::new(origin, direction)
  }

  pub fn render(&self, world: &World) -> Canvas {
    let mut canvas = Canvas::new(self.canvas_width, self.canvas_height);
    for y in 0..self.canvas_height {
      for x in 0..self.canvas_width {
        let ray = self.ray_for_pixel(x, y);
        let colour = world.colour_at(ray);
        canvas.write_pixel(x, y, colour);
      }
    }
    canvas
  }

  pub fn transform(&self) -> &Matrix4x4 {
    &self.transform
  }

  pub fn inverse_transform(&self) -> &Matrix4x4 {
    &self.inverse_transform
  }

  pub fn set_transform(&mut self, transform: Matrix4x4) {
    self.inverse_transform = transform.inverse().unwrap();
    self.transform = transform;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::PI;

  #[test]
  fn construct_camera() {
    let camera = Camera::new(160, 120, PI / 2.0);

    assert_eq!(camera.canvas_width, 160);
    assert_eq!(camera.canvas_height, 120);
    assert!(camera.fov.approx_eq(PI / 2.0));
    assert!(camera.transform.approx_eq(Matrix4x4::IDENTITY));
  }

  #[test]
  fn pixel_size_horizontal_canvas() {
    let camera = Camera::new(200, 125, PI / 2.0);
    assert!(camera.pixel_size.approx_eq(0.01));
  }

  #[test]
  fn pixel_size_vertical_canvas() {
    let camera = Camera::new(125, 200, PI / 2.0);
    assert!(camera.pixel_size.approx_eq(0.01));
  }

  #[test]
  fn construct_ray_center_canvas() {
    let camera = Camera::new(201, 101, PI / 2.0);
    let ray = camera.ray_for_pixel(100, 50);

    let expected = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, -1.0));
    assert!(ray.approx_eq(expected));
  }

  #[test]
  fn construct_ray_corner_canvas() {
    let camera = Camera::new(201, 101, PI / 2.0);
    let ray = camera.ray_for_pixel(0, 0);

    let expected = Ray::new((0.0, 0.0, 0.0), (0.66519, 0.33259, -0.66851));
    assert!(ray.approx_eq(expected));
  }

  #[test]
  fn construct_ray_transformed_camera() {
    let mut camera = Camera::new(201, 101, PI / 2.0);
    camera.transform = Matrix4x4::rotation_y(PI / 4.0) * Matrix4x4::translation(0.0, -2.0, 5.0);

    let ray = camera.ray_for_pixel(100, 50);

    let expected = Ray::new(
      (0.0, 2.0, -5.0),
      (1.0 / 2.0f32.sqrt(), 0.0, -1.0 / 2.0f32.sqrt()),
    );
    println!("     ray: {ray:?}");
    println!("expected: {expected:?}");
    assert!(ray.approx_eq(expected));
  }

  #[test]
  fn render() {
    let world = World::default();
    let mut camera = Camera::new(11, 11, PI / 2.0);
    camera.transform = {
      let from = Point::new(0.0, 0.0, -5.0);
      let to = Point::new(0.0, 0.0, 0.0);
      let up = Vector::new(0.0, 1.0, 0.0);
      Matrix4x4::view_transform(from, to, up)
    };

    let canvas = camera.render(&world);
    let result = canvas.pixel_at(5, 5);
    let expected = Colour::new(0.38066, 0.47583, 0.2855);
    assert!(result.approx_eq(expected));
  }
}
