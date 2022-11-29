use crate::scene::material::Material;
use crate::scene::scene::Mesh;
use crate::scene::texturecoordinate::TextureCoordinate;
use crate::util::vector::Vector;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// A triangle is a part of a mesh, holding the locations of vertices, normals and texture coordinates.
///
/// WARNING: The fields a, b and c are private by design. They represent locations in the mesh' data arrays.
/// They should never be used directly. To get a Triangle's vertices, use `.a()`, `.b()` and `.c()`.
#[derive(Clone)]
pub struct Triangle {
    pub(super) a: usize,
    pub(super) b: usize,
    pub(super) c: usize,

    pub mesh: Arc<Mesh>,
}

impl Debug for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Triangle {{a: {:?}, b: {:?}, c: {:?}}}",
            self.a(),
            self.b(),
            self.c()
        )
    }
}

impl Triangle {
    pub fn get_all(&self) -> (&Vector, &Vector, &Vector) {
        (&self.mesh.vertices[self.a], &self.mesh.vertices[self.b], &self.mesh.vertices[self.c])
    }

    #[inline]
    pub fn a(&self) -> Vector {
        self.mesh.vertices[self.a]
    }

    #[inline]
    pub fn b(&self) -> Vector {
        self.mesh.vertices[self.b]
    }

    #[inline]
    pub fn c(&self) -> Vector {
        self.mesh.vertices[self.c]
    }

    pub fn material(&self) -> Arc<Material> {
        self.mesh.material.clone()
    }

    pub fn normal(&self) -> Vector {
        // TODO: depends on illum model
        let (a, b, c) = self.get_all();
        (*c - *a).cross(&(*c - *b)).unit()
    }

    #[inline]
    pub fn texture_a(&self) -> TextureCoordinate {
        self.mesh.texcoords[self.a]
    }

    #[inline]
    pub fn texture_b(&self) -> TextureCoordinate {
        self.mesh.texcoords[self.b]
    }

    #[inline]
    pub fn texture_c(&self) -> TextureCoordinate {
        self.mesh.texcoords[self.c]
    }

    pub fn area(&self) -> f64 {
        let side1 = (self.c() - self.a()).length();
        let side2 = (self.c() - self.b()).length();
        let side3 = (self.b() - self.a()).length();

        let s = (side1 + side2 + side3) / 2.;

        (s * (s - side1) * (s - side2) * (s - side3)).sqrt()
    }

    pub fn midpoint(&self) -> Vector {
        (self.a() + self.b() + self.c()) / 3.
    }
}
