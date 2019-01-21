
#[macro_export]
macro_rules! att {
    ($( $name: expr => ($data: expr, $no_components: expr)),*) => {{
         let mut vec = Vec::new();
         $( vec.push(crate::core::buffer::Attribute::new($name, $no_components, $data).unwrap()); )*
         vec
    }}
}

pub mod types;
pub mod core;
pub mod objects;
mod loader;

pub mod light;

pub mod camerahandler;
pub mod camera;
pub mod pipeline;

#[cfg(target_os = "emscripten")]
mod emscripten;

pub use gl;
pub use crate::camera::Camera;
pub use crate::texture::Texture;
pub use crate::core::*;
pub use crate::types::*;