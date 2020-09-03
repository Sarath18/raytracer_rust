extern crate image;
mod camera;
mod ray;
mod scene;
mod vector;
mod sphere;

use rand::Rng;
use camera::{Camera, ImageInfo};
use scene::{Scene, World};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use vector::Vector3;
use ray::Ray;
use sphere::{HitRecord, Sphere};

pub fn ray_color(ray: &Ray, world: &World) -> Vector3 {

  let mut rec = HitRecord::init();
  if world.hit(ray, 0.0, std::f64::INFINITY, &mut rec) {
    return 0.5 * (Vector3{x: rec.normal.x, y: rec.normal.y, z: rec.normal.z} + Vector3::from_one(1.0));
  }

  let unit_direction = Vector3::unit_vector(ray.direction);
  let t = 0.5 * unit_direction.y + 1.0;
  return (1.0 - t) * Vector3::from_one(1.0) + t * Vector3{x: 0.5, y: 0.7, z: 1.0};
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
  return if x < min { min } else { if x > max { max } else { x } }
}

pub fn render(scene: &Scene) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(scene.image_info.width, scene.image_info.height);

  let mut rng = rand::thread_rng();

  for y in 0..scene.image_info.height {
    for x in 0..scene.image_info.width {
      let mut pixel_color = Vector3::zero();

      for _ in 0..scene.image_info.samples_per_pixel {
        let u = (x as f64 + rng.gen::<f64>()) / scene.image_info.width as f64;
        let v = (y as f64 + rng.gen::<f64>()) / scene.image_info.height as f64;

        let ray = scene.camera.get_ray(&u, &v);

        let mut color_vec = ray_color(&ray, &scene.world);

        let scale = 1.0 / (scene.image_info.samples_per_pixel as f64);
        color_vec.x = color_vec.x * scale;
        color_vec.y = color_vec.y * scale;
        color_vec.z = color_vec.z * scale;

        pixel_color = pixel_color + color_vec;
      }

      let color = Rgba::from_channels(
        (clamp(pixel_color.x, 0.0, 0.9999) * 256.0) as u8,
        (clamp(pixel_color.y, 0.0, 0.9999) * 256.0) as u8,
        (clamp(pixel_color.z, 0.0, 0.9999) * 256.0) as u8,
        255 as u8
      );

      image.put_pixel(x, scene.image_info.height -1 - y, color);
    }
  }
  return image;
}

fn main() {
  // Image
  let image_info = ImageInfo {
    aspect_ratio: 16.0 / 9.0,
    width: 400,
    height: 225,
    samples_per_pixel: 100
  };

  // Camera
  let mut camera = Camera::init();
  camera.viewport_height = 2.0;
  camera.viewport_width = image_info.aspect_ratio * camera.viewport_height;
  camera.focal_length = 1.0;
  camera.horizonal = Vector3{x: camera.viewport_width, y: 0.0, z: 0.0};
  camera.vertical = Vector3{x: 0.0, y: camera.viewport_height, z: 0.0};
  camera.lower_left_corner = camera.origin - camera.horizonal/2.0 - camera.vertical/2.0 - Vector3{x: 0.0, y: 0.0, z: camera.focal_length};

  let spheres = vec![
    Sphere {
      center: Vector3{x: 0.0, y: 0.0, z: -1.0},
      radius: 0.5
    },
    Sphere {
      center: Vector3{x: 0.0, y: -100.5, z: -1.0},
      radius: 100.0
    },
  ];

  // Scene
  let scene = Scene {
    image_info: image_info,
    camera: camera,
    world: World{spheres: spheres}
  };

  // Render
  let img: DynamicImage = render(&scene);

  img.save("output.png").unwrap();
}
