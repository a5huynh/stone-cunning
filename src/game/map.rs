use amethyst::{
    core::{ transform::Transform },
    prelude::*,
    renderer::{
        SpriteRender,
        SpriteSheetHandle,
        Transparent,
    },
};

use libdwarf::{
    objects::MapObject,
    world::{ Terrain },
};
use crate::game::{
    config::GameConfig,
    entity::{ Floor, Object },

};

#[derive(Debug)]
pub struct PickInfo {
    pub is_terrain: bool,
    pub description: String,
}

/// Map resource used to convert coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
#[derive(Clone)]
pub struct MapResource {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_offset: f32,
    pub world: libdwarf::world::World,
}

impl MapResource {
    pub fn initialize(
        world: &mut World,
        terrain_sprites: SpriteSheetHandle,
        object_sprites: SpriteSheetHandle
    ) -> MapResource {
        let (map_height, map_width, tile_height, tile_width, tile_offset) = {
            let config = &world.read_resource::<GameConfig>();
            (
                config.map_height,
                config.map_width,
                config.tile_height,
                config.tile_width,
                config.tile_offset,
            )
        };

        let mut map_resource = MapResource {
            tile_height: tile_height as f32,
            tile_offset: tile_offset as f32,
            tile_width: tile_width as f32,
            world: libdwarf::world::World::new(map_width, map_height)
        };

        for y in 0..map_height {
            for x in 0..map_width {
                let terrain = map_resource.world.terrain.get(&(x as u32, y as u32)).unwrap();
                let sprite_idx = match terrain {
                    Terrain::STONE => 0,
                    Terrain::MARBLE => 1,
                    Terrain::GRASS => 2,
                    _ => 0,
                };

                let terrain_render = SpriteRender {
                    sprite_sheet: terrain_sprites.clone(),
                    sprite_number: sprite_idx
                };

                world.create_entity()
                    .with(terrain_render)
                    .with(Floor::default())
                    .with(map_resource.place(x as i32, y as i32, 0.0))
                    .with(Transparent)
                    .build();

            }
        }

        let object_render = SpriteRender {
            sprite_sheet: object_sprites.clone(),
            sprite_number: 2,
        };

        let entity = world.create_entity()
            .with(object_render.clone())
            .with(Object::default())
            .with(map_resource.place(5, 5, 1.0))
            .with(Transparent)
            .build();

        map_resource.world.add_object(MapObject::new(entity.id(), 5, 5));
        map_resource
    }

    /// Check to see if there is a collidable object at <x, y>
    pub fn has_collision(&self, map_x: i32, map_y: i32) -> bool {
        // Check if coordinates are outside of bounds
        if map_x < 0 || map_x > self.world.width as i32
            || map_y < 0 || map_y > self.world.height as i32 {
            return false;
        }

        self.world.has_collision(map_x as u32, map_y as u32)
    }

    /// Converts some point <x, y> into map coordinates.
    pub fn to_map_coords(&self, x:f32, y: f32) -> (i32, i32) {
        // Convert position to cartesian coordinates
        let tw = self.tile_width;
        let th = self.tile_height - self.tile_offset;

        let mx = (x / tw) + (y / th);
        let my = (y / th) - (x / tw);

        // Convert cartesian coordinates to map coordinates.
        (
            mx.trunc() as i32,
            my.trunc() as i32
        )
    }

    /// Creates a transform that would place an object on the map using
    /// map coordinates at <x, y> w/ zindex.
    ///
    /// The zoffset is a float, to allow for multiple objects coexisting
    /// on a single tile in a certain order.
    pub fn place(&self, x: i32, y: i32, zoffset: f32) -> Transform {
        let mut transform = Transform::default();

        let px = (x - y) as f32 * self.tile_width / 2.0;
        let py = (x + y) as f32 * (self.tile_height - self.tile_offset) / 2.0;

        let z = -(x + y) as f32;
        transform.set_xyz(px, py, z + zoffset);
        transform
    }

    /// Return information about what's currently at the map coordinates: <x, y>
    pub fn whats_at(&self, x: i32, y: i32) -> Option<PickInfo> {
        let key = (x as u32, y as u32);
        // Any objects at this location?
        if self.world.has_collision(x as u32, y as u32) {
            return Some(
                PickInfo {
                    is_terrain: false,
                    description: format!(
                        "{:?}",
                        self.world.objects_at(x as u32, y as u32)
                    ),
                }
            );
        }

        if self.world.terrain.contains_key(&key) {
            return Some(
                PickInfo {
                    is_terrain: true,
                    description: format!(
                        "{:?}",
                        self.world.terrain.get(&key)
                    )
                }
            );
        }

        return None;
    }
}