use amethyst_imgui::RenderImgui;
use core::amethyst;
use core::amethyst::{
    audio::AudioBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    window::DisplayConfig,
};

mod game;
use game::{config::DwarfConfig, render, state};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_metal", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("resources");

    let display_config_path = assets_dir.join("display_config.ron");
    let binding_path = assets_dir.join("bindings.ron");
    let config_path = assets_dir.join("config.ron");

    let config = DisplayConfig::load(&display_config_path)?;
    let game_config = DwarfConfig::load(&config_path)?;
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(AudioBundle::default())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(render::RenderIso::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;

    let mut game = Application::build(assets_dir, state::MenuState::default())?
        .with_resource(config)
        .with_resource(game_config.game)
        .with_resource(game_config.player)
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}
