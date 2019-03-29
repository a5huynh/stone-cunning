use amethyst::{
    core::{
        transform::Transform,
    },
    ecs::{
        Join,
        ReadExpect,
        System,
        WriteStorage,
    },
    ui::{ UiFinder, UiText },
};

use crate::game::{
    entity::{ CursorSelected, Player },
    map::Map,
};

pub struct DebugUI;
impl<'s> System<'s> for DebugUI {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        UiFinder<'s>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CursorSelected>,
        ReadExpect<'s, Map>,
    );

    fn run(&mut self, (
        mut players,
        mut transforms,
        finder,
        mut ui_text,
        cursor_selected,
        map,
    ): Self::SystemData) {

        let player_loc = (&mut players, &mut transforms).join()
            .next().or(None);

        // Render current player map location
        if let Some((_player, transform)) = player_loc {
            let player_x = transform.translation().x;
            let player_y = transform.translation().y;
            // Convert player position into map coordinates and bump to new location.
            let (map_x, map_y) = map.to_map_coords(player_x, player_y);

            if let Some(entity) = finder.find("player_info") {
                if let Some(text) = ui_text.get_mut(entity) {
                    text.text = format!("Player: ({}, {})", map_x, map_y);
                }
            }
        }

        // Render currently selected info data
        if let Some(entity) = finder.find("debug_info") {
            if let Some(text) = ui_text.get_mut(entity) {
                let selected = &cursor_selected.selected;
                if let Some(pick_info) = selected {
                    text.text = format!(
                        "terrain: {}\ndesc: {}",
                        pick_info.is_terrain,
                        pick_info.description,
                    );
                } else {
                    text.text = String::from("N/A\nN/A");
                }
            }
        }
    }
}