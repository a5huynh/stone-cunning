//! Set of predefined implementations of `RenderPlugin` for use with `RenderingBundle`.
use core::amethyst::{
    ecs::{DispatcherBuilder, World},
    error::Error,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        pass::*,
        rendy::{factory::Factory, graph::render::RenderGroupDesc},
        sprite_visibility::SpriteVisibilitySortingSystem,
        types::Backend,
    },
};

use crate::game::render::pass::DrawSpritesDesc;

/// A [RenderPlugin] for drawing 2d objects with flat shading.
/// Required to display sprites defined with [SpriteRender] component.
#[derive(Default, Debug)]
pub struct RenderSprites {
    target: Target,
}

impl RenderSprites {
    /// Set target to which 2d sprites will be rendered.
    #[allow(dead_code)]
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend> RenderPlugin<B> for RenderSprites {
    fn on_build<'a, 'b>(
        &mut self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &[],
        );
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(RenderOrder::Opaque, DrawFlat2DDesc::new().builder())?;
            ctx.add(RenderOrder::Transparent, DrawSpritesDesc::new().builder())?;
            Ok(())
        });
        Ok(())
    }
}
