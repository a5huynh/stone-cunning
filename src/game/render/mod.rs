pub mod pass;
pub mod pod;

mod sprite;
mod terrain;

pub use sprite::*;
pub use terrain::*;

use core::amethyst::renderer::rendy::shader::{
    ShaderKind, SourceLanguage, SourceShaderInfo, SpirvShader,
};

lazy_static::lazy_static! {
    pub static ref SPRITE_VERTEX: SpirvShader = SourceShaderInfo::new(
        include_str!("../../../resources/shaders/vertex/sprite.vert"),
        "vertex/sprite.vert",
        ShaderKind::Vertex,
        SourceLanguage::GLSL,
        "main"
    ).precompile().unwrap();

    pub static ref SPRITE_FRAGMENT: SpirvShader = SourceShaderInfo::new(
        include_str!("../../../resources/shaders/fragment/sprite.frag"),
        "fragment/sprite.frag",
        ShaderKind::Fragment,
        SourceLanguage::GLSL,
        "main"
    ).precompile().unwrap();
}
