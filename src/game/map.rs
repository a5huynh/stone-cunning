use std::collections::HashMap;
use amethyst::{
    core::{ transform::Transform },
    prelude::*,
    renderer::{
        SpriteRender,
        SpriteSheetHandle,
        Transparent,
    },
};

use crate::game::{
    config::GameConfig,
    entity::{ Floor, Object },

};

// pub const SPRITE_WIDTH: f32 = 32.0;
// pub const SPRITE_HEIGHT: f32 = 32.0;
#[derive(Clone, Debug)]
pub enum Terrain {
    STONE = 0,
    MARBLE = 1,
    GRASS = 2,
    NONE = -1,
}

#[derive(Debug)]
pub struct PickInfo {
    pub is_terrain: bool,
    pub description: String,
}

/// Map resource used to convert coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
#[derive(Clone, Default)]
pub struct Map {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_offset: f32,
    // TODO: Support multiple objects per tile.
    // TODO: Support multi-tile objects.
    pub objects: HashMap<(i32, i32), u32>,
    pub terrain: HashMap<(i32, i32), Terrain>,
}

impl Map {
    pub fn initialize(
        world: &mut World,
        terrain_sprites: SpriteSheetHandle,
        object_sprites: SpriteSheetHandle
    ) -> Map {
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

        let mut map = Map::new(
            tile_width as f32,
            tile_height as f32,
            tile_offset as f32,
        );

        for y in 0..map_height {
            for x in 0..map_width {
                let tile = ((x + y) % 3) as usize;
                let terrain = match tile {
                    0 => Terrain::STONE,
                    1 => Terrain::MARBLE,
                    2 => Terrain::GRASS,
                    _ => Terrain::NONE,
                };

                let terrain_render = SpriteRender {
                    sprite_sheet: terrain_sprites.clone(),
                    sprite_number: tile,
                };

                world.create_entity()
                    .with(terrain_render)
                    .with(Floor::default())
                    .with(map.place(x as i32, y as i32, 0.0))
                    .with(Transparent)
                    .build();

                map.terrain.insert((x as i32, y as i32), terrain);

            }
        }

        let object_render = SpriteRender {
            sprite_sheet: object_sprites.clone(),
            sprite_number: 2,
        };

        world.create_entity()
            .with(object_render.clone())
            .with(Object::default())
            .with(map.place(5, 5, 1.0))
            .with(Transparent)
            .build();

        map.objects.insert((5, 5), 2);
        map
    }

    pub fn new(tile_width: f32, tile_height: f32, tile_offset: f32) -> Self {
        Map {
            tile_width,
            tile_height,
            tile_offset,
            objects: HashMap::new(),
            terrain: HashMap::new(),
        }
    }

    /// Check to see if there is a collidable object at <x, y>
    pub fn has_collision(&self, map_x: i32, map_y: i32) -> bool {
        self.objects.contains_key(&(map_x, map_y))
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
        // Any objects at this location?
        if self.objects.contains_key(&(x, y)) {
            return Some(
                PickInfo {
                    is_terrain: false,
                    description: format!(
                        "{:?}",
                        self.objects.get(&(x, y))
                    ),
                }
            );
        }

        if self.terrain.contains_key(&(x, y)) {
            return Some(
                PickInfo {
                    is_terrain: true,
                    description: format!(
                        "{:?}",
                        self.terrain.get(&(x, y))
                    )
                }
            );
        }

        return None;
    }
}