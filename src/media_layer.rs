use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct ImageContainer<T> {
    pub raw_bytes: Option<&'static [u8]>,
    pub image_data: Cell<Option<T>>,
}

impl<T> ImageContainer<T> {
    pub fn raw_bytes(&self) -> Option<&'static [u8]> {
        self.raw_bytes
    }

    pub fn has_image_data(&self) -> bool {
        let value = self.image_data.take();
        let result = value.is_some();
        self.image_data.set(value);
        result
    }

    pub fn get_image_data(&self) -> Option<T> {
        self.image_data.take()
    }

    pub fn set_image_data(&self, image_data: Option<T>) {
        self.image_data.set(image_data);
    }
}

pub struct ResourceManager<K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<R>,
{
    loader: L,
    cache: RefCell<HashMap<K, Rc<R>>>,
}

impl<K, R, L> ResourceManager<K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<R>,
{
    pub fn new(loader: L) -> Self {
        ResourceManager {
            loader: loader,
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn load<D>(&self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        let mut cache = self.cache.borrow_mut();
        cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

pub trait ResourceLoader<R> {
    type Args: ?Sized;
    fn load(&self, data: &Self::Args) -> Result<R, String>;
}

use packer::Packer;
#[derive(Packer)]
#[packer(source = "images", ignore = "images/.DS_Store")]
pub struct AssetLoader;

impl<T> ResourceLoader<ImageContainer<T>> for AssetLoader {
    type Args = str;
    fn load(&self, name: &str) -> Result<ImageContainer<T>, String> {
        let path = format!("{}{}{}", "images/minesweeper_", name, ".png");
        let raw_bytes: Option<&'static [u8]> = Self::get(&path);
        if raw_bytes.is_some() {
            Ok(ImageContainer {
                raw_bytes: raw_bytes,
                image_data: Cell::new(None),
            })
        } else {
            Err(format!("unable to find image {}", path))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct DummyContainer {}

    impl DummyContainer {
        pub fn new_texture_manager() -> TextureManager {
            let creator = AssetLoader {};
            TextureManager::new(creator)
        }
    }

    #[test]
    fn test_missing() {
        let resource_manager = DummyContainer::new_texture_manager();
        match resource_manager.load("") {
            Ok(_) => assert!(false, "should not have resource"),
            Err(str) => assert_eq!(str, "unable to find image images/minesweeper_.png"),
        };
        match resource_manager.load("missing") {
            Ok(_) => assert!(false, "should not have resource"),
            Err(str) => assert_eq!(str, "unable to find image images/minesweeper_missing.png"),
        };
    }

    #[test]
    fn test_manager() {
        let resource_manager = DummyContainer::new_texture_manager();
        let resource = resource_manager.load("tile");
        assert!(resource.is_ok(), "resource failed to load");

        let container = resource.unwrap();
        assert!(container.raw_bytes().is_some(), "failed to get bytes");
        assert!(!container.has_image_data(), "image should not be ready")
    }
}

#[cfg(feature = "media_layer_sdl2")]
mod sdl2;
#[cfg(feature = "media_layer_sdl2")]
pub type ResourceContainer = self::sdl2::Sdl2Container;
#[cfg(feature = "media_layer_text")]
mod text;
#[cfg(feature = "media_layer_text")]
pub type ResourceContainer = self::text::TextContainer;
#[cfg(feature = "media_layer_wasm")]
mod wasm;
#[cfg(feature = "media_layer_wasm")]
pub type ResourceContainer = self::wasm::WebImageContainer;

pub type Texture = ImageContainer<ResourceContainer>;
pub type TextureManager = ResourceManager<String, Texture, AssetLoader>;
