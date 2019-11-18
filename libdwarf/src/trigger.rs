///
/// Trigger servers two main purposes:
/// - It keeps track of events in the game that agents can respond to
/// - Minimizes the amount of processing the agents need to do to respond
///   to these events.
///
use core::Point3;
use libterrain::Path;

type EntityId = u32;
type MapPosition = Point3<u32>;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TriggerType {
    /// Deals damage to an entity.
    DealDamage {
        target: EntityId,
        source: EntityId,
        damage: i32,
    },
    /// Adds a new entity to the world.
    Add(MapPosition, String),
    /// Adds a new worker to the world.
    AddWorker(MapPosition),
    /// Destroys entities and drops items. Should only be handled by the World.
    Destroy(EntityId),
    // Harvest a resource, e.g. chopping wood.
    HarvestResource {
        target: EntityId,
        position: MapPosition,
        resource: String,
    },
    /// Take an object and place into inventory.
    /// NOTE: No checks are made to see if the entity is actually nearby or not.
    Take { target: EntityId, owner: EntityId },
    /// Move along path
    Move(Path),
    /// Move to some location.
    MoveTo(MapPosition),
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
