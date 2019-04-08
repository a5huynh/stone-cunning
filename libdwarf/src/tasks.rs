#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Idle dwarf
    Chilling,
    // Harvest a resource, e.g. chopping wood.
    HarvestResource((u32, u32), String, u32),
    /// Move to some location.
    MoveTo(u32, u32),
}