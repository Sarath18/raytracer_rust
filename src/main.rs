extern crate image;
mod camera;
mod ray;
mod scene;
mod vector;
mod sphere;

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

pub fn render(scene: &Scene) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(scene.image_info.width, scene.image_info.height);

  for y in 0..scene.image_info.height {
    for x in 0..scene.image_info.width {
      let u = x as f64 / scene.image_info.width as f64;
      let v = y as f64 / scene.image_info.height as f64;

      let ray = Ray {
        origin: scene.camera.origin,
        direction: scene.camera.lower_left_corner + u * scene.camera.horizonal + v * scene.camera.vertical - scene.camera.origin
      };

      let color_vec = ray_color(&ray, &scene.world);

      let color = Rgba::from_channels(
        (color_vec.x * 255.0) as u8,
        (color_vec.y * 255.0) as u8,
        (color_vec.z * 255.0) as u8,
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
    height: 225
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
