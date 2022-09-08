use raytracer_challenge::canvas::Canvas;
use std::fs::File;
use std::io::Write;

fn main() {
  let mut canvas = Canvas::new(10, 2);
  let colour = (1.0, 0.0, 0.0).into();
  for y in 0..2 {
    for x in 0..10 {
      canvas.write_pixel(x, y, colour);
    }
  }

  let image_buffer = canvas.to_image();
  image_buffer.save("test.png").unwrap();

  let ppm = canvas.to_ppm().unwrap();
  let mut ppm_output = File::create("test.ppm").unwrap();
  write!(ppm_output, "{ppm}").unwrap();
}
