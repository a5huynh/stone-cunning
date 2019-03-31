use std::collections::VecDeque;
pub mod world;


pub enum Job {
    // Idle dwarf
    Chilling,
    // Chopping wood
    ChopWood,
    // Mining stuff
    MineRock,
}

pub struct Tasks {
    pub queue: VecDeque<Job>,
}

