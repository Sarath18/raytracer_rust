extern crate image;

use image::{DynamicImage, GenericImage, Pixel, Rgba};

pub struct Scene {
  pub width: u32,
  pub height: u32,
}

pub fn render(scene: &Scene) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
  
  for y in 0..scene.height {
    for x in 0..scene.width {
      
      let color = Rgba::from_channels(
        (x as f32 / scene.height as f32 * 255.0) as u8,
        (y as f32 / scene.width as f32 * 255.0) as u8,
        64 as u8,
        255 as u8,
      );

      image.put_pixel(x, y, color);
    }
  }

  return image;
}

fn main() {
  let scene = Scene {
    width: 800,
    height: 600
  };

  let img: DynamicImage = render(&scene);
  img.save("output.png").unwrap();
}
