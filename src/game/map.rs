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
                let terrain_render = SpriteRender {
                    sprite_sheet: terrain_sprites.clone(),
                    sprite_number: ((x + y) % 3) as usize,
                };

                world.create_entity()
                    .with(terrain_render)
                    .with(Floor::default())
                    .with(map.place(x as i32, y as i32, 0.0))
                    .with(Transparent)
                    .build();
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
            objects: HashMap::new()
        }
    }

    /// Check to see if there is a collidable object at <x, y>
    pub fn has_collision(&self, x: f32, y: f32) -> bool {
        let (map_x, map_y) = self.to_map_coords(x, y);
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
}