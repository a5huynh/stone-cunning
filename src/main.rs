use amethyst::{
    assets::Processor,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::{ReadExpect, Resources, SystemData},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        pass::DrawFlat2DTransparentDesc, types::DefaultBackend, Factory, Format, GraphBuilder,
        GraphCreator, Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder,
        sprite_visibility::SpriteVisibilitySortingSystem,
    },
    ui::{DrawUiDesc, UiBundle},
    utils::{application_root_dir, fps_counter::FPSCounterBundle},
    window::{DisplayConfig, ScreenDimensions, Window, WindowBundle},
};

use libdwarf::systems;

mod game;
use game::{
    config::DwarfConfig,
    state::RunningState,
    systems::{
        ui::debug::DebugUI, CursorSystem, MapMovementSystem, PlayerMovement, RenderNPCSystem,
        RenderObjectSystem,
    },
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("resources").join("display_config.ron");
    let binding_path = app_root.join("resources").join("bindings.ron");
    let config_path = app_root.join("resources").join("config.ron");

    let config = DisplayConfig::load(&display_config_path);
    let game_config = DwarfConfig::load(&config_path);
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with_bundle(FPSCounterBundle::default())?
        // Register the systems, give it a name, and specify any
        // dependencies for that system.
        .with_bundle(input_bundle)?
        // Simulation systems.
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with(systems::AssignTaskSystem, "assign_task", &[])
        .with(systems::WorkerSystem, "worker_sim", &["assign_task"])
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
        // Moving around the map
        .with(MapMovementSystem, "map_movement", &[])
        .with(PlayerMovement, "player_movement", &[])
        // Should always be last so we have the most up-to-date info.
        .with(
            DebugUI::default(),
            "debug_ui",
            &["cursor", "player_movement"],
        )
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));

    let mut game = Application::build("./", RunningState::default())?
        .with_resource(config)
        .with_resource(game_config.game)
        .with_resource(game_config.player)
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();

    Ok(())
}

// This graph structure is used for creating a proper `RenderGraph` for rendering.
// A renderGraph can be thought of as the stages during a render pass. In our case,
// we are only executing one subpass (DrawFlat2D, or the sprite pass). This graph
// also needs to be rebuilt whenever the window is resized, so the boilerplate code
// for that operation is also here.
#[derive(Default)]
struct RenderGraph {
    dimensions: Option<ScreenDimensions>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for RenderGraph {
    // This trait method reports to the renderer if the graph must be rebuilt, usually because
    // the window has been resized. This implementation checks the screen size and returns true
    // if it has changed.
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    // This is the core of a RenderGraph, which is building the actual graph with subpasses and target
    // images.
    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        self.dirty = false;

        // Retrieve a reference to the target window, which is created by the WindowBundle
        let window = <ReadExpect<'_, Window>>::fetch(res);
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        // Create a new drawing surface in our window
        let surface = factory.create_surface(&window);
        let surface_format = factory.get_surface_format(&surface);

        // Begin building our RenderGraph
        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            // clear screen to black
            Some(ClearValue::Color([0.0, 0.0, 0.0, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let sprite_pass = graph_builder.add_node(
            SubpassBuilder::new()
                .with_group(DrawFlat2DTransparentDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let ui_pass = graph_builder.add_node(
            SubpassBuilder::new()
                .with_dependency(sprite_pass)
                .with_group(DrawUiDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        // Finally, add the pass to the graph
        let _present = graph_builder.add_node(
            PresentNode::builder(factory, surface, color)
                .with_dependency(sprite_pass)
                .with_dependency(ui_pass),
        );

        graph_builder
    }
}
