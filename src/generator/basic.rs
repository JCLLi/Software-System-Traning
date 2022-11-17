use crate::generator::{Callback, Generator};

use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

#[derive(Debug)]
pub struct BasicGenerator;

impl Generator for BasicGenerator {
    fn generate(&self, camera: &Camera, callback: &Callback) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height, "backup.rgb");

        for x in 0..camera.width {
            for y in 0..camera.height {
                let res = callback(x, y);
                output.set_at(x, y, res);
            }
        }

        output
    }
}
