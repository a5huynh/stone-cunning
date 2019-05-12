use crate::game::{config::GameConfig, sprite::SpriteSheetStorage};
use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use libdwarf::resources::Map;
use libterrain::Biome;

/// Map resource used to convert coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
pub struct MapRenderer {
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_offset: f32,
}

impl MapRenderer {
    pub fn initialize(world: &mut World) -> Self {
        let (tile_height, tile_width, tile_offset) = {
            let config = &world.read_resource::<GameConfig>();
            (config.tile_height, config.tile_width, config.tile_offset)
        };

        let map_render = MapRenderer {
            tile_height: tile_height as f32,
            tile_offset: tile_offset as f32,
            tile_width: tile_width as f32,
        };

        // Load terrain map from sim
        let (sprite_sheet, object_sheet) = {
            let sheets = world.read_resource::<SpriteSheetStorage>();
            (sheets.terrain.clone(), sheets.object.clone())
        };

        let (terrain, width, height) = {
            let map = world.read_resource::<Map>();
            (map.terrain.clone(), map.width, map.height)
        };

        // Create terrain
        for y in 0..height {
            for x in 0..width {
                let sprite_idx = match terrain.get_biome(x as u32, y as u32) {
                    Biome::TAIGA => 0,
                    Biome::SNOW => 1,
                    Biome::GRASSLAND => 2,
                    Biome::OCEAN => 3,
                    Biome::BEACH => 4,
                    _ => 0,
                };

                let terrain_render = SpriteRender {
                    sprite_sheet: sprite_sheet.clone(),
                    sprite_number: sprite_idx,
                };

                world
                    .create_entity()
                    .with(terrain_render)
                    .with(map_render.place(x as i32, y as i32, 0, 0.0))
                    .with(Transparent)
                    .build();
            }
        }

        // Add objects to map
        for (pos, _) in terrain.objects().iter() {
            let sprite = SpriteRender {
                sprite_sheet: object_sheet.clone(),
                sprite_number: 2,
            };

            world
                .create_entity()
                .with(sprite)
                .with(map_render.place(pos.x as i32, pos.y as i32, 0, 0.0))
                .with(Transparent)
                .build();
        }

        map_render
    }

    /// Converts some point <x, y> into map coordinates.
    pub fn to_map_coords(&self, x: f32, y: f32) -> (i32, i32) {
        // Convert position to cartesian coordinates
        let tw = self.tile_width;
        let th = self.tile_height - self.tile_offset;

        let mx = (x / tw) + (y / th);
        let my = (y / th) - (x / tw);

        // Convert cartesian coordinates to map coordinates.
        (mx.trunc() as i32, my.trunc() as i32)
    }

    /// Creates a transform that would place an object on the map using
    /// map coordinates at <x, y> w/ zindex.
    ///
    /// The zoffset is a float, to allow for multiple objects coexisting
    /// on a single tile in a certain order.
    pub fn place(&self, x: i32, y: i32, z: i32, zoffset: f32) -> Transform {
        let mut transform = Transform::default();

        let px = (x - y) as f32 * self.tile_width / 2.0;
        let py = (x + y) as f32 * (self.tile_height - self.tile_offset) / 2.0
            - z as f32 * (self.tile_height);
        let pz = -(x + y) as f32 + zoffset;

        transform.set_xyz(px, py, pz);
        transform
    }
}
