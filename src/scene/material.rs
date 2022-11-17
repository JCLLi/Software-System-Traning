use crate::scene::texture::{Texture, TextureAtlas};
use crate::util::vector::Vector;

use once_cell::sync::Lazy;
use std::sync::Arc;

pub static DEFAULT_MATERIAL: Lazy<Arc<Material>> = Lazy::new(|| {
    Arc::new(Material {
        name: Default::default(),
        ambient: Vector::default(),
        diffuse: Vector::default(),
        specular: Vector::default(),
        shininess: 0.0,
        dissolve: 0.0,
        optical_density: 0.0,
        ambient_texture: None,
        diffuse_texture: None,
        specular_texture: None,
        normal_texture: None,
        dissolve_texture: None,
        illumination_model: None,

        emittance: Vector::default(),
        emittance_texture: None,
    })
});

#[derive(Debug)]
pub struct Material {
    /// Material name as specified in the MTL file
    pub name: String,
    /// Ambient color of the material
    pub ambient: Vector,
    /// Diffuse color of the material
    pub diffuse: Vector,
    /// Specular color of the material
    pub specular: Vector,
    /// Material shininess attribute
    pub shininess: f64,
    /// Dissolve attribute is the alpha term for the material. Referred to as dissolve since that's
    /// what the MTL file format docs refer to it as
    pub dissolve: f64,
    /// Optical density also known as index of refraction. Called optical_density in the MTL specc.
    /// Takes on a value between 0.001 and 10.0. 1.0 means light does not bend as it passed through
    /// the object.
    pub optical_density: f64,
    /// Name of the ambient texture file for the material. No path is pre-pended to the texture
    /// file names specified in the MTL file
    pub ambient_texture: Option<Texture>,
    /// Name of the diffuse texture file for the material. No path is pre-pended to the texture
    /// file names specified in the MTL file
    pub diffuse_texture: Option<Texture>,
    /// Name of the specular texture file for the material. No path is pre-pended to the texture
    /// file names specified in the MTL file
    pub specular_texture: Option<Texture>,
    /// Name of the normal map texture file for the material. No path is pre-pended to the texture
    /// file names specified in the MTL file
    pub normal_texture: Option<Texture>,
    /// Name of the alpha map texture file for the material. No path is pre-pended to the texture
    /// file names specified in the MTL file. Referred to as dissolve to match the MTL file format
    /// specification
    pub dissolve_texture: Option<Texture>,
    /// The illumination model to use for this material. The different illumination models are
    /// specified in http://paulbourke.net/dataformats/mtl/
    pub illumination_model: Option<u8>,

    pub emittance: Vector,
    pub emittance_texture: Option<Texture>,
}

impl Material {
    pub(super) fn from_tobj_material(
        material: tobj::Material,
        textureatlas: Arc<TextureAtlas>,
    ) -> Self {
        let default_emittance = "0.0 0.0 0.0".into();
        let default_emittance_texture_name = "".into();

        let stremittance = material
            .unknown_param
            .get("Ke")
            .unwrap_or(&default_emittance);
        let emittancevec: Vec<f64> = stremittance
            .split(" ")
            .map(|i| i.parse())
            .collect::<Result<Vec<f64>, _>>()
            .unwrap_or(vec![0., 0., 0.]);

        let emittance = if emittancevec.len() != 3 {
            Vector::new(0., 0., 0.)
        } else {
            Vector::new(emittancevec[0], emittancevec[1], emittancevec[2])
        };

        let emittance_texture_name = material
            .unknown_param
            .get("map_Ke")
            .unwrap_or(&default_emittance_texture_name);

        Self {
            name: material.name,
            ambient: Vector::from_arr(material.ambient),
            diffuse: Vector::from_arr(material.diffuse),
            specular: Vector::from_arr(material.specular),
            shininess: material.shininess as f64,
            dissolve: material.dissolve as f64,
            optical_density: material.optical_density as f64,
            ambient_texture: textureatlas.get_texture(&material.ambient_texture),
            diffuse_texture: textureatlas.get_texture(&material.diffuse_texture),
            specular_texture: textureatlas.get_texture(&material.specular_texture),
            normal_texture: textureatlas.get_texture(&material.normal_texture),
            dissolve_texture: textureatlas.get_texture(&material.dissolve_texture),
            illumination_model: material.illumination_model,

            emittance,
            emittance_texture: textureatlas.get_texture(&emittance_texture_name),
        }
    }
}
