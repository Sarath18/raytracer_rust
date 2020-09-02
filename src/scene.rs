use crate::camera::{Camera, ImageInfo};

#[derive(Copy, Clone, Debug)]
pub struct Scene {
  pub image_info: ImageInfo,
  pub camera: Camera
}
