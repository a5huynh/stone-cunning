use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        SpriteRender,
        Transparent,
    },
};

use crate::game::{
    config::GameConfig,
    math::cart2iso,
};

// TODO: Make more generalized?
#[derive(Default)]
pub struct DwarfNPC;

impl Component for DwarfNPC {
    type Storage = DenseVecStorage<Self>;
}

impl DwarfNPC {
    pub fn initialize(world: &mut World, npc_sprite: SpriteSheetHandle) {
        world.register::<DwarfNPC>();

        let (tile_height, tile_width, tile_scale) = {
            let config = &world.read_resource::<GameConfig>();
            (
                config.tile_height,
                config.tile_width,
                config.tile_scale
            )
        };

        let scaled_width = (tile_width as f32 * tile_scale) / 2.0;
        let scaled_height = (tile_height as f32 * tile_scale) / 2.0;
        let mut transform = Transform::default();

        let cart_x = 2.0 * scaled_width;
        let cart_y = 2.0 * scaled_height;
        let (iso_x, iso_y) = cart2iso(cart_x, cart_y);

        transform.set_xyz(iso_x, iso_y, -2.0);
        transform.set_scale(tile_scale, tile_scale, 1.0);

        let sprite_render = SpriteRender {
            sprite_sheet: npc_sprite.clone(),
            sprite_number: 0,
        };

        world.create_entity()
            .with(sprite_render.clone())
            .with(DwarfNPC::default())
            .with(transform)
            .with(Transparent)
            .build();
    }
}