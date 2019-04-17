use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        SpriteRender,
        Transparent,
    },
};

use libdwarf::world::WorldSim;

use crate::game::{
    map::MapResource,
};

// TODO: Make more generalized?
#[derive(Default)]
pub struct DwarfNPC {
    sim_ref: u32
}

impl Component for DwarfNPC {
    type Storage = DenseVecStorage<Self>;
}

impl DwarfNPC {
    pub fn initialize(world: &mut World, sim: &mut WorldSim, map: &mut MapResource, npc_sprite: SpriteSheetHandle) {
        world.register::<DwarfNPC>();

        let sim_ref = sim.add_worker(2, 2);

        let entity = world.create_entity()
            .with(SpriteRender {
                sprite_sheet: npc_sprite.clone(),
                sprite_number: 0,
            })
            .with(DwarfNPC { sim_ref })
            .with(map.place(2, 2, 1.0))
            .with(Transparent)
            .build();

    }
}