use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    window::DisplayConfig,
};
use amethyst_imgui::RenderImgui;

use libdwarf::systems;

mod game;
use game::{
    config::DwarfConfig,
    state::RunningState,
    systems::{
        ui::debug::DebugUI, ClickSystem, CursorSystem, MapMovementSystem, PlayerMovement,
        RenderNPCSystem, RenderObjectSystem,
    },
};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_metal", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("resources").join("display_config.ron");
    let binding_path = app_root.join("resources").join("bindings.ron");
    let config_path = app_root.join("resources").join("config.ron");

    let config = DisplayConfig::load(&display_config_path);
    let game_config = DwarfConfig::load(&config_path);
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        // Register the systems, give it a name, and specify any
        // dependencies for that system.
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?
        // Simulation systems.
        .with(systems::WorkerSystem, "worker_sim", &[])
        .with(systems::ObjectSystem, "object_sim", &[])
        .with(
            systems::WorldUpdateSystem::default(),
            "world_updates",
            &["worker_sim", "object_sim"],
        )
        .with(systems::TimeTickSystem, "game_tick", &["world_updates"])
        // Render systems. Takes entities from the simulations and assigns sprites
        // to them as they get added.
        .with(RenderObjectSystem, "render_obj_system", &["world_updates"])
        .with(RenderNPCSystem, "render_npc_system", &["world_updates"])
        // Cursor selection
        .with(CursorSystem, "cursor", &[])
        // We handle click after the cursor is correctly transformed on the map.
        .with(ClickSystem, "click", &["cursor"])
        // Moving around the map
        .with(MapMovementSystem, "map_movement", &[])
        .with(PlayerMovement, "player_movement", &[])
        // Should always be last so we have the most up-to-date info.
        .with(
            DebugUI::default(),
            "debug_ui",
            &["cursor", "player_movement"],
        );

    let mut game = Application::build("./", RunningState::default())?
        .with_resource(config)
        .with_resource(game_config.game)
        .with_resource(game_config.player)
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}
