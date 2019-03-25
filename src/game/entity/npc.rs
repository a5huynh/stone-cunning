use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        SpriteRender,
        Transparent,
    },
};

use crate::game::{
    map::Map,
};

// TODO: Make more generalized?
#[derive(Default)]
pub struct DwarfNPC;

impl Component for DwarfNPC {
    type Storage = DenseVecStorage<Self>;
}

impl DwarfNPC {
    pub fn initialize(world: &mut World, map: &Map, npc_sprite: SpriteSheetHandle) {
        world.register::<DwarfNPC>();

        let mut transform = map.place(2.0, 2.0, 1.0);
        transform.set_scale(map.tile_scale, map.tile_scale, map.tile_scale);

        world.create_entity()
            .with(SpriteRender {
                sprite_sheet: npc_sprite.clone(),
                sprite_number: 0,
            })
            .with(DwarfNPC::default())
            .with(transform)
            .with(Transparent)
            .build();
    }
}