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
    math::{ cart2iso, iso2cart },
};

/// Map resource used to convert coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
#[derive(Clone, Default)]
pub struct Map {
    pub tile_width: f32,
    pub tile_height: f32,
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
        let (map_height, map_width, tile_height, tile_width) = {
            let config = &world.read_resource::<GameConfig>();
            (
                config.map_height,
                config.map_width,
                config.tile_height,
                config.tile_width
            )
        };

        let mut map = Map::new(
            tile_width as f32,
            tile_height as f32,
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
                    .with(map.place(x as f32, y as f32, 0.0))
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
            .with(map.place(5.0, 5.0, 1.0))
            .with(Transparent)
            .build();

        map.objects.insert((5, 5), 2);
        map
    }

    pub fn new(tile_width: f32, tile_height: f32) -> Self {
        Map {
            tile_width,
            tile_height,
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
        let (cartx, carty) = iso2cart(x, y);
        // Convert cartesian coordinates to map coordinates.
        (
            (cartx / self.tile_width * 2.0) as i32,
            (carty / self.tile_height * 2.0) as i32
        )
    }

    pub fn place(&self, x: f32, y: f32, zindex: f32) -> Transform {
        let mut transform = Transform::default();

        let (iso_x, iso_y) = cart2iso(x * self.tile_width / 2.0, y * self.tile_height / 2.0);

        transform.set_xyz(iso_x, iso_y, -(x + y) + zindex);
        transform
    }
}