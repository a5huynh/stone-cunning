use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ALPHA,
    ColorMask,
    DisplayConfig,
    DrawFlat2D,
    Pipeline,
    RenderBundle,
    Stage,
};
use amethyst::ui::{ DrawUi, UiBundle };
use amethyst::utils::application_root_dir;

mod game;
use game::{
    config::DwarfConfig,
    state::RunningState,
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
    // let binding_path = format!("{}/bindings.ron", resource_root);
    let config_path = format!("{}/config.ron", resource_root);

    let config = DisplayConfig::load(&path);
    let game_config = DwarfConfig::load(&config_path);
    let input_bundle = InputBundle::<String, String>::new();
        // .with_bindings_from_file(binding_path)?;

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
                .with_pass(DrawUi::new())
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        // Register the systems, give it a name, and specify any
        // dependencies for that system.
        .with_bundle(input_bundle)?;

    let mut game = Application::build("./", RunningState)?
        .with_resource(game_config.game)
        // .with_resource(game_config.enemy)
        // .with_resource(game_config.game)
        // .with_resource(game_config.player)
        .build(game_data)?;

    game.run();

    Ok(())
}
