use rand::rngs::OsRng;
use rand::Rng;

pub mod camera;
pub mod color;
pub mod consts;
pub mod outputbuffer;
pub mod ray;
pub mod vector;

pub fn get_rng() -> impl Rng {
    // Best random distribution
    OsRng
}
