use std::collections::HashMap;

use specs::{world::Builder, World};

use crate::{
    config::ResourceConfig,
    components::{MapObject, MapPosition},
};
use libterrain::{Biome, Object, Point3, TerrainGenerator};

pub struct Map {
    // TODO: Support multiple objects per tile.
    pub object_map: HashMap<Point3<u32>, u32>,
    /// Location map of all the workers.
    pub worker_map: HashMap<Point3<u32>, u32>,
    pub terrain: TerrainGenerator,
    // World dimensions
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn initialize(world: &mut World, width: u32, height: u32) -> Self {
        let terrain = TerrainGenerator::new(width, height).build();
        let mut object_map = HashMap::new();

        let resource_map = {
            let resources = world.read_resource::<ResourceConfig>();
            resources.map.clone()
        };

        // Initialize map w/ objects created in terrain gen
        for (pos, object) in &terrain.objects() {
            let mut entity_builder = world.create_entity();
            entity_builder = match object {
                Object::TREE => {
                    let resource = resource_map.get("tree").unwrap().clone();
                    entity_builder.with(MapObject::new(&resource))
                }
            };

            entity_builder = entity_builder.with(MapPosition { pos: *pos });
            let entity = entity_builder.build();
            object_map.insert(*pos, entity.id());
        }

        Map {
            object_map,
            worker_map: HashMap::new(),
            terrain,
            width,
            height,
        }
    }

    pub fn is_inside_map(&self, pt: Point3<i32>) -> bool {
        pt.x >= 0 && pt.x < self.width as i32 && pt.y >= 0 && pt.y < self.height as i32
    }

    /// Find the north, east, south, west neighboring objects for some
    /// point <x, y>.
    pub fn find_neighbors(&self, pt: Point3<u32>) -> Vec<&u32> {
        // Generate the coordinates for the neighbors
        let mut neighbor_idx = Vec::with_capacity(4);
        neighbor_idx.push(Point3::new(pt.x, pt.y + 1, pt.z));
        neighbor_idx.push(Point3::new(pt.x + 1, pt.y, pt.z));
        if pt.y > 0 {
            neighbor_idx.push(Point3::new(pt.x, pt.y - 1, pt.z));
        }
        if pt.x > 0 {
            neighbor_idx.push(Point3::new(pt.x - 1, pt.y, pt.z));
        }

        // Find the neighbors and return the results
        let mut results = Vec::new();
        for idx in &neighbor_idx {
            if let Some(oid) = self.object_map.get(idx) {
                results.push(oid);
            }
        }
        results
    }

    pub fn has_collision(&self, pt: Point3<i32>) -> bool {
        if self.is_inside_map(pt) {
            let key = Point3::new(pt.x as u32, pt.y as u32, pt.z as u32);
            return self.object_map.contains_key(&key) || self.worker_map.contains_key(&key);
        }

        false
    }

    pub fn objects_at(&self, pt: Point3<i32>) -> Option<u32> {
        if self.is_inside_map(pt) {
            let key = Point3::new(pt.x as u32, pt.y as u32, pt.z as u32);
            if let Some(id) = self.object_map.get(&key) {
                return Some(*id);
            }
        }

        None
    }

    pub fn terrain_at(&self, pt: Point3<i32>) -> Option<Biome> {
        if self.is_inside_map(pt) {
            self.terrain
                .get_biome(pt.x as u32, pt.y as u32, pt.z as u32)
        } else {
            None
        }
    }

    pub fn worker_at(&self, pt: Point3<i32>) -> Option<u32> {
        if self.is_inside_map(pt) {
            let key = Point3::new(pt.x as u32, pt.y as u32, pt.z as u32);
            if let Some(id) = self.worker_map.get(&key) {
                return Some(*id);
            }
        }

        None
    }

    pub fn move_worker(&mut self, entity: u32, old_pt: Point3<u32>, new_pt: Point3<u32>) {
        self.worker_map.remove(&old_pt);
        self.track_worker(entity, new_pt);
    }

    pub fn remove_object(&mut self, _entity: u32, pt: Point3<u32>) {
        self.object_map.remove(&pt);
    }

    pub fn track_object(&mut self, entity: u32, pt: Point3<u32>) {
        self.object_map.insert(pt, entity);
    }

    pub fn track_worker(&mut self, entity: u32, pt: Point3<u32>) {
        self.worker_map.insert(pt, entity);
    }
}
