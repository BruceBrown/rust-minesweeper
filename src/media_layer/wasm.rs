use super::AssetLoader;
use super::TextureManager;

use image;
use wasm_bindgen::Clamped;
extern crate web_sys;

pub struct WebImageContainer {
    width: u32,
    height: u32,
    raw_bytes: Vec<u8>,
    image_data: web_sys::ImageData,
}

use image::imageops;
impl WebImageContainer {
    pub fn new(raw_bytes: &'static [u8], width: u32, height: u32) -> Self {
        match image::load_from_memory_with_format(raw_bytes, image::ImageFormat::Png) {
            Ok(image) => {
                let mut raw_bytes =
                    imageops::resize(&image, width, height, imageops::FilterType::Gaussian)
                        .to_vec();
                        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                            Clamped(raw_bytes.as_mut_slice()),
                            width,
                            height,
                        ).unwrap();
                
                Self {
                    width,
                    height,
                    raw_bytes,
                    image_data,
                }
            }
            Err(_e) => {
                let image_data = web_sys::ImageData::new_with_sw(
                    width,
                     height,
                 ).unwrap();
                Self {
 
                width,
                height,
                raw_bytes: Vec::new(),
                image_data,
                }
            },
        }
    }

    pub fn get_image_data(&self) -> &web_sys::ImageData {
        &self.image_data
    }

    pub fn new_texture_manager() -> TextureManager {
        let creator = AssetLoader {};
        TextureManager::new(creator)
    }
}
