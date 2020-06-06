///
/// Trigger servers two main purposes:
/// - It keeps track of events in the game that agents can respond to
/// - Minimizes the amount of processing the agents need to do to respond
///   to these events.
///
use core::{EntityId, WorldPos};
use libterrain::Path;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TriggerType {
    /// Deals damage to an entity.
    DealDamage {
        target: EntityId,
        source: EntityId,
        damage: i32,
    },
    /// Adds a new entity to the world.
    Add(WorldPos, String),
    /// Adds a new worker to the world.
    AddWorker(WorldPos),
    /// Destroys entities and drops items. Should only be handled by the World.
    Destroy(EntityId),
    // Harvest a resource, e.g. chopping wood.
    HarvestResource {
        target: EntityId,
        position: WorldPos,
        resource: String,
    },
    /// Take an object and place into inventory.
    /// NOTE: No checks are made to see if the entity is actually nearby or not.
    Take { target: EntityId, owner: EntityId },
    /// Move along path
    Move(Path),
    /// Move to some location.
    MoveTo(WorldPos),
}

pub enum TriggerPriority {
    LOW,
    MEDIUM,
    HIGH,
}

pub struct TriggerRecord {
    pub trigger: TriggerType,
    pub source: usize,
    pub priority: TriggerPriority,
}
