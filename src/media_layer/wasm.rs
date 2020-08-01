use super::AssetLoader;
use super::TextureManager;

use image;
use wasm_bindgen::Clamped;
extern crate web_sys;

pub struct WebImageContainer {
    width: u32,
    height: u32,
    raw_bytes: Vec<u8>,
}

use image::imageops;
impl WebImageContainer {
    pub fn new(raw_bytes: &'static [u8], width: u32, height: u32) -> Self {
        match image::load_from_memory_with_format(raw_bytes, image::ImageFormat::Png) {
            Ok(image) => {
                let raw_bytes =
                    imageops::resize(&image, width, height, imageops::FilterType::Gaussian)
                        .to_vec();
                Self {
                    width,
                    height,
                    raw_bytes: raw_bytes,
                }
            }
            Err(_e) => Self {
                width,
                height,
                raw_bytes: Vec::new(),
            },
        }
    }

    pub fn get_image_data(&self) -> Result<web_sys::ImageData, wasm_bindgen::JsValue> {
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(self.raw_bytes.clone().as_mut_slice()),
            self.width,
            self.height,
        )
    }

    pub fn new_texture_manager() -> TextureManager {
        let creator = AssetLoader {};
        TextureManager::new(creator)
    }
}
