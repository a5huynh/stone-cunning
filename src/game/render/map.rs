use crate::game::{config::GameConfig, sprite::SpriteSheetStorage};
use core::amethyst::{
    core::{math::Point3, transform::Transform},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use libdwarf::resources::Map;
use libterrain::Biome;

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

/// Map resource used to convert coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
pub struct MapRenderer {
    pub tile_width: f32,
    pub tile_height: f32,
}

impl MapRenderer {
    pub fn initialize(world: &mut World) -> Self {
        let (tile_height, tile_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.tile_height, config.tile_width)
        };

        let map_render = MapRenderer {
            tile_height: tile_height as f32,
            tile_width: tile_width as f32,
        };

        // Load terrain map from sim
        let sprite_sheet = {
            let sheets = world.read_resource::<SpriteSheetStorage>();
            sheets.terrain.clone()
        };

        let (terrain, width, height) = {
            let map = world.read_resource::<Map>();
            (map.terrain.clone(), map.width, map.height)
        };

        for y in 0..height {
            for x in 0..width {
                for z in 32..64 {
                    let pt = Point3::new(x, y, z);
                    if let Some(biome) = terrain.get(x as u32, y as u32, z as u32) {
                        let mut block = world.create_entity();
                        if terrain.is_visible(x as u32, y as u32, z as u32) {
                            let sprite_idx = match biome {
                                Biome::TAIGA => 0,
                                Biome::SNOW | Biome::TUNDRA => 1,
                                Biome::GRASSLAND => 2,
                                Biome::OCEAN => 3,
                                Biome::BEACH => 4,
                                Biome::ROCK => 5,
                            };

                            let terrain_render = SpriteRender {
                                sprite_sheet: sprite_sheet.clone(),
                                sprite_number: sprite_idx,
                            };

                            block = block.with(terrain_render);
                        }

                        block
                            .with(map_render.place(&pt, 0.0, Direction::NORTH))
                            .with(Transparent)
                            .build();
                    }
                }
            }
        }

        map_render
    }

    /// Converts some point <x, y> into map coordinates.
    pub fn to_map_coords(&self, x: f32, y: f32) -> (i32, i32) {
        // Convert position to cartesian coordinates
        let tw = self.tile_width;
        let th = self.tile_height;

        let mx = (x / tw) + (y / th);
        let my = (y / th) - (x / tw);

        // Convert cartesian coordinates to map coordinates.
        (mx.trunc() as i32, my.trunc() as i32)
    }

    /// Creates a transform that would place an object on the map using
    /// map coordinates at <x, y, z> w/ zindex.
    ///
    /// The zoffset is a float, to allow for multiple objects coexisting
    /// on a single tile in a certain order.
    pub fn place(&self, pt: &Point3<u32>, zoffset: f32, direction: Direction) -> Transform {
        let mut transform = Transform::default();

        let fx = pt.x as f32;
        let fy = pt.y as f32;
        let fz = pt.z as f32;

        let mut px = 0.0;
        let mut py = 0.0;

        match direction {
            Direction::NORTH => {
                px = (fx - fy) * self.tile_width / 2.0;
                py = (fx + fy) * self.tile_height / 2.0 + (fz * self.tile_height);
            }
            Direction::EAST => {
                px = (fx - fy) * self.tile_width / 2.0;
                py = (fx + fy) * self.tile_height / 2.0 + (fz * self.tile_height);
            }
            Direction::SOUTH => {
                px = (fx - fy) * self.tile_width / 2.0;
                py = (fx + fy) * self.tile_height / 2.0 + (fz * self.tile_height);
            }
            _ => {}
        };

        let pz = -(fx + fy) + zoffset;

        transform.set_translation_xyz(px, py, pz);
        transform
    }
}
