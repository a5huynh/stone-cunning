use amethyst::{
    core::{
        frame_limiter::FrameRateLimitStrategy,
        transform::TransformBundle,
    },
    input::InputBundle,
    prelude::*,
    renderer::{
        ALPHA,
        ColorMask,
        DisplayConfig,
        DrawFlat2D,
        Pipeline,
        RenderBundle,
        Stage,
    },
    ui::{ DrawUi, UiBundle },
    utils::{
        application_root_dir,
        fps_counter::FPSCounterBundle,
    },
};

use libdwarf::systems;

mod game;
use game::{
    config::DwarfConfig,
    state::RunningState,
    systems::{
        CursorSystem,
        RenderObjectSystem,
        RenderNPCSystem,
        MapMovementSystem,
        PlayerMovement,
        ui::debug::DebugUI,
    }
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(amethyst::LoggerConfig {
        stdout: amethyst::StdoutLog::Colored,
        level_filter: amethyst::LogLevelFilter::Error,
        log_file: None,
        allow_env_override: true
    });

    let resource_root = format!("{}/resources", application_root_dir());
    let path = format!("{}/display_config.ron", resource_root);
    let binding_path = format!("{}/bindings.ron", resource_root);
    let config_path = format!("{}/config.ron", resource_root);

    let config = DisplayConfig::load(&path);
    let game_config = DwarfConfig::load(&config_path);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    // Setup the rendering pipeline
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 0.0], 1.0)
                // Draw sprites on a 2D quad.
                .with_pass(DrawFlat2D::new()
                    .with_transparency(
                        ColorMask::all(),
                        ALPHA,
                        None
                    )
                )
                // Draw mesh without any lighting.
                // .with_pass(DrawFlat::<PosTex>::new())
                // Should always be the last pass in the pipeline.
                .with_pass(DrawUi::new())
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config.clone()))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(FPSCounterBundle::default())?
        // Register the systems, give it a name, and specify any
        // dependencies for that system.
        .with_bundle(input_bundle)?
        // Simulation systems.
        .with(systems::AssignTaskSystem, "assign_task", &[])
        .with(systems::WorkerSystem, "worker_sim", &["assign_task"])
        .with(systems::ObjectSystem, "object_sim", &[])
        .with(systems::WorldUpdateSystem::default(), "world_updates", &["worker_sim", "object_sim"])
        // Render systems. Takes entities from the simulations and assigns sprites
        // to them as they get added.
        .with(RenderObjectSystem, "render_obj_system", &["world_updates"])
        .with(RenderNPCSystem, "render_npc_system", &["world_updates"])
        // Cursor selection
        .with(CursorSystem, "cursor", &[])
        // Moving around the map
        .with(MapMovementSystem, "map_movement", &[])
        .with(PlayerMovement, "player_movement", &[])
        // Should always be last so we have the most up-to-date info.
        .with(DebugUI, "debug_ui", &["cursor", "player_movement"]);

    let mut game = Application::build("./", RunningState)?
        .with_resource(config)
        .with_resource(game_config.game)
        .with_resource(game_config.player)
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}
