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
#[derive(Default)]
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
    ) {
        let (map_height, map_width, tile_height, tile_width, tile_scale) = {
            let config = &world.read_resource::<GameConfig>();
            (
                config.map_height,
                config.map_width,
                config.tile_height,
                config.tile_width,
                config.tile_scale
            )
        };

        let scaled_width = (tile_width as f32 * tile_scale) / 2.0;
        let scaled_height = (tile_height as f32 * tile_scale) / 2.0;

        for y in 0..map_height {
            for x in 0..map_width {
                let terrain_render = SpriteRender {
                    sprite_sheet: terrain_sprites.clone(),
                    sprite_number: ((x + y) % 3) as usize,
                };

                let cart_x = x as f32 * scaled_width;
                let cart_y = y as f32 * scaled_height;
                let zindex = (x + y) as f32;
                let (iso_x, iso_y) = cart2iso(cart_x, cart_y);

                let mut transform = Transform::default();
                // Add tile offset as config option.
                transform.set_xyz(iso_x, iso_y, -zindex);
                transform.set_scale(tile_scale, tile_scale, tile_scale);

                world.create_entity()
                    .with(terrain_render)
                    .with(Floor::default())
                    .with(transform)
                    .with(Transparent)
                    .build();
            }
        }

        let object_render = SpriteRender {
            sprite_sheet: object_sprites.clone(),
            sprite_number: 2,
        };

        let cart_x = 5.0 * scaled_width;
        let cart_y = 5.0 * scaled_height;
        let (iso_x, iso_y) = cart2iso(cart_x, cart_y);

        let mut transform = Transform::default();
        transform.set_xyz(iso_x, iso_y + 32.0, -9.0);
        transform.set_scale(tile_scale, tile_scale, tile_scale);
        world.create_entity()
            .with(object_render.clone())
            .with(Object::default())
            .with(transform)
            .with(Transparent)
            .build();

        let mut map = Map::new(
            scaled_width,
            scaled_height
        );
        map.objects.insert((5, 5), 2);
        world.add_resource(map);
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
            (cartx / self.tile_width) as i32,
            (carty / self.tile_height) as i32
        )
    }
}