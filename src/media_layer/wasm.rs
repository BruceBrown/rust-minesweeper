use super::AssetLoader;
use super::TextureManager;

use image::imageops::{resize, FilterType};
use wasm_bindgen::Clamped;
use web_sys::ImageData;

/**
 * The WebImageContainer carries the ImageData as well as the raw_bytes that underpin it.
 */
pub struct WebImageContainer {
    raw_bytes: Vec<u8>,
    image_data: ImageData,
}

impl WebImageContainer {
    pub fn new(raw_bytes: &'static [u8], width: u32, height: u32) -> Self {
        match image::load_from_memory_with_format(raw_bytes, image::ImageFormat::Png) {
            Ok(image) => {
                let mut raw_bytes = resize(&image, width, height, FilterType::Gaussian).to_vec();
                let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                    Clamped(raw_bytes.as_mut_slice()),
                    width,
                    height,
                )
                .unwrap();
                Self {
                    raw_bytes,
                    image_data,
                }
            }
            Err(_e) => {
                let image_data = ImageData::new_with_sw(width, height).unwrap();
                Self {
                    raw_bytes: Vec::new(),
                    image_data,
                }
            }
        }
    }

    pub fn get_image_data(&self) -> &ImageData {
        &self.image_data
    }

    pub fn new_texture_manager() -> TextureManager {
        let creator = AssetLoader {};
        TextureManager::new(creator)
    }
}
