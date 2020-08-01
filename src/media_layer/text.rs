use super::AssetLoader;
use super::TextureManager;

pub struct TextContainer {}

impl TextContainer {
    pub fn new_texture_manager() -> TextureManager<Self> {
        let creator = AssetLoader {};
        TextureManager::new(creator)
    }
}
