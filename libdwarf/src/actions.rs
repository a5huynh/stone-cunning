use libpath::Path;
use libterrain::Point3;

type EntityId = u32;
type MapPosition = Point3<u32>;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ActionType {
    /// Idle dwarf
    Chilling,
    /// Deals damage to an entity.
    DealDamage(EntityId, i32),
    /// Adds a new entity to the world.
    Add(MapPosition, String),
    /// Adds a new worker to the world.
    AddWorker(MapPosition),
    /// Destroys entities and drops items. Should only be handled by the World.
    Destroy(EntityId),
    // Harvest a resource, e.g. chopping wood.
    HarvestResource(MapPosition, String, String),
    /// Take an object and place into inventory.
    /// NOTE: No checks are made to see if the entity is actually nearby or not.
    Take {
        target: EntityId,
        owner: EntityId,
    },
    /// Move along path
    Move(Path),
    /// Move to some location.
    MoveTo(MapPosition),
}
