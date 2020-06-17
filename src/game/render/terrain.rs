//! Set of predefined implementations of `RenderPlugin` for use with `RenderingBundle`.
use core::amethyst::{
    ecs::{prelude::*, DispatcherBuilder, ReadExpect, ReadStorage, World},
    error::Error,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        rendy::{factory::Factory, graph::render::RenderGroupDesc},
        types::Backend,
    },
};

use crate::game::render::pass::DrawTerrainDesc;
use libdwarf::components::{EntityInfo, Terrain};

/// A [RenderPlugin] for drawing 2d objects with flat shading.
/// Required to display sprites defined with [SpriteRender] component.
#[derive(Default, Debug)]
pub struct RenderTerrain {
    target: Target,
}

type TerrainSetupData<'a> = (ReadStorage<'a, EntityInfo>, ReadStorage<'a, Terrain>);

impl RenderTerrain {
    /// Set target to which 2d sprites will be rendered.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend> RenderPlugin<B> for RenderTerrain {
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Since render passes at any time, we need to make sure it has access to
        // components that may be used. This initializes them in `specs`.
        TerrainSetupData::setup(world);
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(RenderOrder::Transparent, DrawTerrainDesc::new().builder())?;
            Ok(())
        });
        Ok(())
    }
}
