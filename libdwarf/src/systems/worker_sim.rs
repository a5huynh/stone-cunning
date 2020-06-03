use core::amethyst::ecs::{
    Entities, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage,
};

use crate::{
    components::{EntityInfo, MapObject, Worker, WorkerAction},
    config::WorldConfig,
    planner::{Condition, Planner, State},
    resources::{time::Time, TaskQueue},
    trigger::TriggerType,
};
use core::utils::is_near;
use libterrain::TerrainLoader;

pub struct WorkerSystem;
impl<'a> System<'a> for WorkerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, MapObject>,
        WriteStorage<'a, EntityInfo>,
        WriteExpect<'a, TerrainLoader>,
        WriteExpect<'a, Planner>,
        Write<'a, TaskQueue>,
        ReadExpect<'a, Time>,
        ReadExpect<'a, WorldConfig>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut workers,
            objects,
            mut entity_infos,
            mut map,
            planner,
            mut tasks,
            time,
            config,
        ): Self::SystemData,
    ) {
        for (_entity, worker, entity_info) in (&*entities, &mut workers, &mut entity_infos).join() {
            // Regen worker energy.
            if worker.energy < config.worker_stamina {
                // NOTE: This might need to be revisited. Pausing the simulation would
                // mean a large `time.delta_seconds()` which immediately resets
                // the worker energy, leading to a burst of actions for a couple frames
                // before going back to normal.
                worker.energy = (worker.energy + (config.worker_stamina * time.delta_seconds()))
                    .min(config.worker_stamina);
            }

            if worker.energy < config.action_cost {
                continue;
            }

            // Assign new task if the current action was completed.
            if worker.current_action.is_none() {
                let current_pos = &entity_info.pos;
                let mut state = State::new();
                worker.add_state(&mut state);

                if let Some(TriggerType::HarvestResource {
                    target,
                    position,
                    resource,
                }) = tasks.worker.pop_front()
                {
                    // Are we already near the resource?
                    state.insert(
                        Condition::Near(resource.clone()),
                        is_near(&position, &current_pos),
                    );

                    // Create the desired state
                    let mut desired_state = State::new();
                    desired_state.insert(Condition::Has(resource.clone()), true);

                    // Plan stuff
                    let mut planned = planner.plan(&state, &desired_state);
                    while let Some(action) = planned.pop() {
                        // Convert planned actions into actions for the worker.
                        let entity = entities.entity(target);
                        worker.queue.push_back(WorkerAction {
                            target: Some(entity.id()),
                            action: action.clone(),
                            target_pos: position,
                        });
                    }
                }

                worker.current_action = worker.queue.pop_front();
            }

            // Process current worker action
            if worker.current_action.is_some() {
                // Grab the latest target info, if any.
                let mut target_obj = None;
                if let Some(target_id) = worker.current_action.as_ref().unwrap().target {
                    let entity = entities.entity(target_id);
                    target_obj = objects.get(entity);
                }

                worker.do_work(&mut tasks, &mut map, entity_info, target_obj);
            }

            worker.energy -= config.action_cost;
        }
    }
}
