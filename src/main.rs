#![allow(unused)]

use raytracer_challenge::*;
use std::f32::consts::PI;
use std::{fs::File, io::Write};

fn main() {
  chapter_8();
}

fn chapter_8() {
  let world = {
    let mut world = World::new();

    let floor = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::scale(10.0, 0.01, 10.0);
      sphere.material = Material {
        colour: (1.0, 0.9, 0.9).into(),
        specular: 0.0,
        ..Default::default()
      };
      sphere
    };

    let left_wall = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(0.0, 0.0, 5.0)
        * Matrix4x4::rotation_y(-PI / 4.0)
        * Matrix4x4::rotation_x(PI / 2.0)
        * Matrix4x4::scale(10.0, 0.01, 10.0);
      sphere.material = floor.material.clone();
      sphere
    };

    let right_wall = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(0.0, 0.0, 5.0)
        * Matrix4x4::rotation_y(PI / 4.0)
        * Matrix4x4::rotation_x(PI / 2.0)
        * Matrix4x4::scale(10.0, 0.01, 10.0);
      sphere.material = floor.material.clone();
      sphere
    };

    let middle = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(-0.5, 1.0, 0.5);
      sphere.material = Material {
        colour: (0.1, 1.0, 0.5).into(),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
      };
      sphere
    };

    let right = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(1.5, 1.0, -0.5) * Matrix4x4::scale(0.5, 0.5, 0.5);
      sphere.material = Material {
        colour: (0.5, 1.0, 0.1).into(),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
      };
      sphere
    };

    let left = {
      let mut sphere = Sphere::new();
      sphere.transform =
        Matrix4x4::translation(-1.5, 0.33, -0.75) * Matrix4x4::scale(0.33, 0.33, 0.33);
      sphere.material = Material {
        colour: (1.0, 0.8, 0.1).into(),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
      };
      sphere
    };

    let light = PointLight::new((-10.0, 10.0, -10.0), (1.0, 1.0, 1.0));

    for object in [floor, left_wall, right_wall, middle, right, left] {
      world.objects.push(Box::new(object));
    }
    world.lights.push(light);

    world
  };

  let camera = {
    let mut camera = Camera::new(800, 600, PI / 3.0);
    camera.set_transform(Matrix4x4::view_transform(
      (0.0, 1.5, -5.0),
      (0.0, 1.0, 00.0),
      (0.0, 1.0, 0.0),
    ));
    camera
  };

  let canvas = camera.render(&world);
  canvas.to_image().save("chapter_8.png").unwrap();
}

fn chapter_6() {
  let ray_origin = Point::from((0.0, 0.0, -5.0));
  let wall_z = 10.0;
  let wall_size = 7.0;
  let canvas_height = 400;

  let pixel_size = wall_size / canvas_height as f32;
  let half_wall_size = wall_size / 2.0;

  let mut canvas = Canvas::new(canvas_height, canvas_height);

  let light = PointLight::new((-10.0, 10.0, -10.0), (1.0, 1.0, 1.0));

  let mut shape = Sphere::new();
  shape.material = Material {
    colour: (1.0, 0.2, 1.0).into(),
    ..Default::default()
  };

  for y in 0..canvas.height {
    let world_y = half_wall_size - pixel_size * y as f32;
    for x in 0..canvas.width {
      let world_x = pixel_size * x as f32 - half_wall_size;
      let position = Point::from((world_x, world_y, wall_z));
      let ray = Ray::new(ray_origin, (position - ray_origin).normalise());
      let xs = shape.intersect(ray);
      let hit = xs.hit();

      if let Some(hit) = hit {
        let colour = {
          let point = ray.position(hit.t);
          let normal = hit.object.normal_at(point);
          let eye = -ray.direction;

          hit
            .object
            .material()
            .lighting(&light, point, eye, normal, false)
        };

        canvas.write_pixel(x, y, colour)
      }
    }
  }

  canvas.to_image().save("chapter_6.png").unwrap();
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
