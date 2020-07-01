pub mod pass;
pub mod pod;

mod sprite;
mod terrain;

pub use sprite::*;
pub use terrain::*;

use core::amethyst::renderer::rendy::{hal::pso::ShaderStageFlags, shader::SpirvShader};

lazy_static::lazy_static! {
    pub static ref SPRITE_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../../resources/shaders/compiled/sprite.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main"
    ).unwrap();

    pub static ref SPRITE_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../../resources/shaders/compiled/sprite.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main"
    ).unwrap();

    pub static ref ISO_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../../resources/shaders/compiled/iso.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main"
    ).unwrap();

    pub static ref ISO_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../../resources/shaders/compiled/iso.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main"
    ).unwrap();
}
