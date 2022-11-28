use crate::scene::error::SceneError;
use crate::scene::material::{Material, DEFAULT_MATERIAL};
use crate::scene::texture::TextureAtlasBuilder;
use crate::scene::texturecoordinate::TextureCoordinate;
use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use std::path::PathBuf;

use std::fmt;

use std::fmt::{Debug, Formatter};
use std::hash::Hasher;

use once_cell::sync::OnceCell;
use std::sync::Arc;

pub struct FastHash(u64);
impl Hasher for FastHash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for i in bytes {
            self.0 |= *i as u64
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vector>,
    pub normals: Vec<Vector>,
    pub triangles: OnceCell<Vec<Arc<Triangle>>>,
    pub texcoords: Vec<TextureCoordinate>,

    pub material: Arc<Material>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: vec![],
            normals: vec![],
            triangles: OnceCell::new(),
            texcoords: vec![],
            material: DEFAULT_MATERIAL.clone(),
        }
    }
}

pub struct Scene {
    meshes: Vec<Arc<Mesh>>,
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<Scene...>")
    }
}

impl Scene {
    pub fn triangles(&self) -> impl Iterator<Item = Arc<Triangle>> + '_ {
        self.meshes
            .iter()
            .flat_map(move |i| i.triangles.get().unwrap().to_vec())
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vector> + '_ {
        self.meshes.iter().flat_map(move |i| i.vertices.to_vec())
    }

    pub fn texture_coordinates(&self) -> impl Iterator<Item = TextureCoordinate> + '_ {
        self.meshes.iter().flat_map(move |i| i.texcoords.to_vec())
    }

    pub fn normals(&self) -> impl Iterator<Item = Vector> + '_ {
        self.meshes.iter().flat_map(move |i| i.normals.to_vec())
    }
}

/// Builds a scene from an object representation. For example [tobj](docs.rs/tobj)
#[derive(Debug,Default)]
pub struct SceneBuilder {
    /// This path is used to search for texture files.
    texturepath: PathBuf,
}

impl SceneBuilder {
    pub fn texturepath(mut self, path: PathBuf) -> Self {
        self.texturepath = path;
        self
    }

    pub fn build_from_tobj(
        &self,
        (models, tobjmaterials): (Vec<tobj::Model>, Vec<tobj::Material>),
    ) -> Result<Scene, SceneError> {
        //creating meshes is efficient enough
        let mut meshes: Vec<_> = (0..models.len())
            .map(|_| Arc::new(Mesh::default()))
            .collect();
        let mut textureatlasbuilder = TextureAtlasBuilder::new();

        //might be able to improve with lazy iterator
        for material in &tobjmaterials {
            if !material.diffuse_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.diffuse_texture, &self.texturepath)?
            }
            if !material.ambient_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.ambient_texture, &self.texturepath)?
            }
            if !material.dissolve_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.dissolve_texture, &self.texturepath)?
            }
            if !material.specular_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.specular_texture, &self.texturepath)?
            }

            //might be an improvement, don't know yet
            // let default_emittance_texture_name = "".into();
            let default_emittance_texture_name = String::from("");
            let emittance_texture_name = material
                .unknown_param
                .get("map_Ke")
                .unwrap_or(&default_emittance_texture_name);

            if !emittance_texture_name.is_empty() {
                textureatlasbuilder.add_texture_file(emittance_texture_name, &self.texturepath)?
            }
        }

        let textureatlas = Arc::new(textureatlasbuilder.build());

        let mut materials = Vec::new();

        for i in tobjmaterials {
            materials.push(Arc::new(Material::from_tobj_material(
                i,
                textureatlas.clone(),
            )));
        }

        for (index, model) in models.iter().enumerate() {
            let vertices = model
                .mesh
                .positions
                .chunks_exact(3)
                .map(|i| Vector::new(i[0] as f64, i[1] as f64, i[2] as f64))
                .collect::<Vec<_>>();
            let normals = model
                .mesh
                .normals
                .chunks_exact(3)
                .map(|i| Vector::new(i[0] as f64, i[1] as f64, i[2] as f64))
                .collect::<Vec<_>>();
            let texcoords = model
                .mesh
                .texcoords
                .chunks_exact(2)
                .map(|i| TextureCoordinate::new(i[0] as f64, i[1] as f64))
                .collect::<Vec<_>>();

            let material = match model.mesh.material_id {
                Some(id) => materials[id].clone(),
                None => DEFAULT_MATERIAL.clone(),
            };

            let mesh = Arc::new(Mesh {
                vertices,
                triangles: OnceCell::new(),
                normals,
                texcoords,
                material,
            });

            let triangles = model
                .mesh
                .indices
                .chunks_exact(3)
                .map(|i| {
                    Arc::new(Triangle {
                        a: i[0] as usize,
                        b: i[1] as usize,
                        c: i[2] as usize,
                        mesh: mesh.clone(),
                    })
                })
                .collect::<Vec<_>>();

            mesh.triangles.set(triangles).unwrap();

            meshes[index] = mesh;
        }

        Ok(Scene { meshes })
    }
}
