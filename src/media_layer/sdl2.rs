use super::AssetLoader;
use super::TextureManager;

pub struct Sdl2Container {
    pub texture: sdl2::render::Texture,
}

impl Sdl2Container {
    pub fn new(texture: sdl2::render::Texture) -> Self {
        Self { texture }
    }
    pub fn new_texture_manager() -> TextureManager {
        let creator = AssetLoader {};
        TextureManager::new(creator)
    }
}
