use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;

pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

pub struct Texture<'l> {
    pub texture: sdl2::render::Texture<'l>,
}

impl<'l> Texture<'l> {
    pub fn new(texture: sdl2::render::Texture<'l>) -> Self {
        Self { texture }
    }
}

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: 'l + ResourceLoader<'l, R>,
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'l, R>,
{
    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;
    fn load(&'l self, name: &str) -> Result<Texture<'l>, String> {
        let path = format!("{}{}{}", "images/minesweeper_", name, ".png");
        match Assets::get(&path) {
            Some(data) => {
                let loader = sdl2::rwops::RWops::from_bytes(data)?;
                let ops = &loader as &dyn sdl2::image::ImageRWops;
                let surface = ops.load_png()?;
                match self.create_texture_from_surface(&surface) {
                    Ok(texture) => Ok(Texture::new(texture)),
                    Err(e) => Err(format!("yikes: {}", e)),
                }
            }
            None => {
                println!("packer failed to load image {}", path);
                Ok(Texture::new(self.load_texture(path)?))
            }
        }
    }
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

use packer::Packer;
#[derive(Packer)]
#[packer(source = "images", ignore = "images/.DS_Store")]
struct Assets;
