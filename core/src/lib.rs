pub use amethyst;
pub use amethyst::core::math::{Point3, Vector2, Vector3};
pub use log;
pub mod utils;
pub use uuid::Uuid;

/// Alias around ids used in the ECS world which only exists during run-time,
/// not to be confused with an entity's UUID which represents game objects throughout
/// time.
pub type EntityId = u32;
pub type WorldPos = Point3<i32>;
