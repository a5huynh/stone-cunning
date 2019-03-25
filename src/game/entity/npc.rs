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

        world.create_entity()
            .with(SpriteRender {
                sprite_sheet: npc_sprite.clone(),
                sprite_number: 0,
            })
            .with(DwarfNPC::default())
            .with(map.place(2, 2, 1.0))
            .with(Transparent)
            .build();
    }
}