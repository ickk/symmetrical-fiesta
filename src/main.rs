#![allow(unused)]

use raytracer_challenge::*;

use std::f32::consts::PI;
use std::{fs::File, io::Write};

fn main() {
  chapter_5();
}

fn chapter_5() {
  let ray_origin = Point::from((0.0, 0.0, -5.0));
  let wall_z = 10.0;
  let wall_size = 7.0;
  let canvas_height = 400;

  let pixel_size = wall_size / canvas_height as f32;
  let half_wall_size = wall_size / 2.0;

  let mut canvas = Canvas::new(canvas_height, canvas_height);
  let colour = Colour::RED;

  let mut shape = Sphere::new();
  shape.transform = Matrix4x4::translation(0.3, 0.2, 0.0)
    * Matrix4x4::rotation_z(PI / 3.0)
    * Matrix4x4::scale(1.0, 0.5, 1.0);

  for y in 0..canvas.height {
    let world_y = half_wall_size - pixel_size * y as f32;
    for x in 0..canvas.width {
      let world_x = pixel_size * x as f32 - half_wall_size;
      let position = Point::from((world_x, world_y, wall_z));
      let ray = Ray::new(ray_origin, (position - ray_origin).normalise());
      let xs = shape.intersect(ray);
      let hit = xs.hit();

      if hit.is_some() {
        canvas.write_pixel(x, y, colour)
      }
    }
  }

  canvas.to_image().save("chapter_5.png").unwrap();
  // let ppm = canvas.to_ppm().unwrap();
  // write!(File::create("chapter_5.ppm").unwrap(), "{ppm}");
}
