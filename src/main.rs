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

mod game;
use game::{
    config::DwarfConfig,
    state::RunningState,
    systems,
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
        // Cursor selection
        .with(systems::CursorSystem, "cursor", &[])
        .with(systems::MapMovementSystem, "map_movement", &[])
        .with(systems::PlayerMovement, "player_movement", &[])
        // Handles syncing rendering front-end w/ simulation
        // TODO: Combine into a single system?
        .with(systems::NPCSim, "npc_sim", &[])
        // Should always be last so we have the most up-to-date info.
        .with(systems::ui::debug::DebugUI, "debug_ui", &["cursor", "player_movement"]);

    let mut game = Application::build("./", RunningState::default())?
        .with_resource(config)
        .with_resource(game_config.game)
        .with_resource(game_config.player)
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}
