use crate::scene::texture::{Texture, TextureError};
use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;

pub struct TextureAtlasBuilder {
    atlas: HashMap<String, Texture>,
}

impl TextureAtlasBuilder {
    pub fn new() -> Self {
        Self {
            atlas: HashMap::new(),
        }
    }

    pub fn add_texture_file(
        &mut self,
        filename: impl AsRef<Path>,
        basepath: impl AsRef<Path>,
    ) -> Result<(), TextureError> {
        self.add_texture(
            filename
                .as_ref()
                .to_str()
                .ok_or(TextureError::FileName)?
                .into(),
            Texture::new(basepath.as_ref().join(filename))?,
        );
        Ok(())
    }

    pub fn add_texture(&mut self, name: String, texture: Texture) {
        self.atlas.insert(name, texture);
    }

    pub fn build(self) -> TextureAtlas {
        let mut atlas = HashMap::new();
        let atlassize = self.atlas.len();

        let mut textures = vec![
            Texture {
                image: DynamicImage::new_rgb8(0, 0).into_rgb8(),
                size: (0, 0),
            };
            atlassize
        ];

        let mut names = Vec::with_capacity(atlassize);

        for (index, (key, value)) in self.atlas.into_iter().enumerate() {
            textures[index] = value;
            names.push(key);
        }

        for (index, name) in names.into_iter().enumerate() {
            let mesh = textures[index].clone();
            atlas.insert(name, mesh);
        }

        TextureAtlas { atlas }
    }
}

pub struct TextureAtlas {
    pub(self) atlas: HashMap<String, Texture>,
}

impl TextureAtlas {
    pub fn get_texture(&self, name: &String) -> Option<Texture> {
        self.atlas.get(name).cloned()
    }
}
