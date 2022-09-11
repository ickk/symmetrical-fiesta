#![allow(unused)]

use perlin2d::PerlinNoise2D;
use raytracer_challenge::*;
use std::f32::consts::PI;
use std::{fs::File, io::Write};

fn main() {
  chapter_11();
}

fn chapter_11() {
  let world = {
    let mut world = World::new();

    let floor = {
      let mut plane = Plane::new();
      plane.material = Material {
        pattern: Pattern::checkers((0.9, 0.9, 0.9).into(), (0.7, 0.7, 0.7).into())
          .with_transform(Matrix4x4::scale(1.5, 1.5, 1.5)),
        specular: 0.0,
        reflective: 0.3,
        shininess: 0.3,
        ..Default::default()
      };
      plane
    };

    let middle = {
      let perlin_obj = PerlinNoise2D::new(6, 10.0, 10.0, 1.0, 2.0, (100.0, 100.0), 1.0, 97);

      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(-0.5, 1.0, 0.5)
        * Matrix4x4::rotation_z(2.5 * PI / 3.0)
        * Matrix4x4::rotation_x(3.5 * PI / 4.0)
        * Matrix4x4::rotation_y(0.6 * PI / 4.0);
      sphere.material = Material {
        pattern: Pattern::perlin(
          perlin_obj,
          0.05,
          PatternType::Stripe((0.1, 1.0, 0.5).into(), (0.4, 0.8, 0.8).into()),
        )
        .with_transform(
          Matrix4x4::rotation_x(PI / 4.0)
            * Matrix4x4::rotation_y(-PI / 4.0)
            * Matrix4x4::scale(0.2, 0.2, 0.2),
        ),
        diffuse: 0.7,
        specular: 0.3,
        reflective: 0.1,
        ..Default::default()
      };
      sphere
    };

    let right = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(1.5, 0.5, 2.3) * Matrix4x4::scale(0.5, 0.5, 0.5);
      sphere.material = Material {
        pattern: Pattern::gradient((0.5, 0.0, 0.1).into(), (0.5, 0.5, 0.9).into())
          .with_transform(Matrix4x4::rotation_z(3.0 * PI / 5.0) * Matrix4x4::scale(0.5, 1.0, 1.0)),
        diffuse: 0.7,
        specular: 0.3,
        reflective: 0.05,
        ..Default::default()
      };
      sphere
    };

    let left = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(-1.7, 3.1, -1.3)
        * Matrix4x4::rotation_x(PI / 6.0)
        * Matrix4x4::scale(0.33, 0.33, 0.33);
      sphere.material = Material {
        pattern: Pattern::ring((1.0, 0.4, 0.1).into(), (0.9, 0.1, 0.1).into())
          .with_transform(Matrix4x4::scale(0.25, 0.25, 0.25)),
        diffuse: 0.7,
        specular: 0.3,
        reflective: 0.1,
        ..Default::default()
      };
      sphere
    };

    let hemisphere = {
      let mut sphere = Sphere::new();
      sphere.transform = Matrix4x4::translation(0.8, 0.1, 4.0) * Matrix4x4::scale(0.5, 0.5, 0.5);
      sphere.material = Material {
        pattern: Pattern::solid((0.3, 0.4, 0.7).into()),
        diffuse: 0.1,
        specular: 0.9,
        reflective: 0.9,
        ..Default::default()
      };
      sphere
    };

    let light = PointLight::new((-10.0, 10.0, -10.0), Colour::new(0.94, 0.9, 0.83));

    world.objects.push(Box::new(floor));
    for object in [middle, right, left, hemisphere] {
      world.objects.push(Box::new(object));
    }
    world.lights.push(light);

    world
  };

  let camera = {
    let mut camera = Camera::new(1920, 1080, PI / 6.0);
    camera.set_transform(Matrix4x4::view_transform(
      (-1.5, 6.0, -5.0),
      (-0.5, 1.5, 0.0),
      (0.0, 1.0, 0.0),
    ));
    camera
  };

  eprintln!("start time: {}\n", chrono::Utc::now().to_rfc2822());
  let start = std::time::Instant::now();
  let img = camera.render_img(&world);
  eprintln!("\nelapsed: {:.2?}\n", std::time::Instant::now() - start);
  eprintln!("saving..");
  img.save("chapter_11_fhd.png").unwrap();
  eprintln!("done.");
  eprintln!("\ntotal time: {:.2?}", std::time::Instant::now() - start);
}
