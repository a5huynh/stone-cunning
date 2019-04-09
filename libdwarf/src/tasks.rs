#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Idle dwarf
    Chilling,
    /// Deals damage to an entity.
    DealDamage(u32),
    /// Destroys entities and drops items. Should only be handled by the World.
    Destroy(u32),
    // Harvest a resource, e.g. chopping wood.
    HarvestResource((u32, u32), String, u32),
    /// Take an object and place into inventory.
    Take(u32),
    /// Move to some location.
    MoveTo(u32, u32),
}