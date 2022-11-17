use crate::scene::texture::TextureError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SceneError {
    #[error(transparent)]
    TextureError(#[from] TextureError),
}
