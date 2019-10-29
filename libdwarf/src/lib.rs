pub mod components;
pub mod config;
pub mod planner;
pub mod resources;
pub mod systems;
pub mod trigger;
pub mod world;

use core::amethyst::core::{
    ecs::{DispatcherBuilder, World},
    SystemBundle,
};

#[derive(Default)]
pub struct WorldSimBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for WorldSimBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), core::amethyst::Error> {
        builder.add(systems::WorkerSystem, "worker_sim", &[]);
        builder.add(systems::ObjectSystem, "object_sim", &[]);
        builder.add(
            systems::WorldUpdateSystem::default(),
            "world_updates",
            &["worker_sim", "object_sim"],
        );

        builder.add(systems::TimeTickSystem, "game_tick", &["world_updates"]);

        Ok(())
    }
}
