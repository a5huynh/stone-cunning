type EntityId = u32;
type MapPosition = (u32, u32);

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Idle dwarf
    Chilling,
    /// Deals damage to an entity.
    DealDamage(u32),
    /// Adds a new entity to the world.
    Add(MapPosition, EntityId),
    /// Destroys entities and drops items. Should only be handled by the World.
    Destroy(EntityId),
    // Harvest a resource, e.g. chopping wood.
    HarvestResource(MapPosition, String, EntityId),
    /// Take an object and place into inventory.
    Take(EntityId),
    /// Move to some location.
    MoveTo(u32, u32),
}