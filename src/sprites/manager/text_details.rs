use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct StringCreator {}
impl<'l> ResourceLoader<'l, Texture<'l>> for StringCreator {
    type Args = str;
    fn load(&'l self, name: &str) -> Result<Texture<'l>, String> {
        Ok(Texture::new(name.to_string()))
    }
}

pub struct Texture<'l> {
    pub texture: String,
    marker: std::marker::PhantomData<&'l String>,
}

impl<'l> Texture<'l> {
    pub fn new(texture: String) -> Self {
        Self {
            texture,
            marker: std::marker::PhantomData {},
        }
    }
}

pub type TextureManager<'l> = ResourceManager<'l, String, Texture<'l>, StringCreator>;

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

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}
