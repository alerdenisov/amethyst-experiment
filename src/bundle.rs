use components::{Position, Velocity};
use systems::{ApplyVelocity, UpdateTransform, CollideSystem, FPSOutput};

use amethyst::Result;
use amethyst::ecs::ECSBundle;
use amethyst::prelude::*;
use amethyst::timing::Time;

use amethyst::ecs::util::resources::FPSCounter;
use amethyst::ecs::util::systems::FPSCounterSystem;


pub struct GameBundle;

impl<'a, 'b, T> ECSBundle<'a, 'b, T> for GameBundle {
    fn build(
        &self,
        builder: ApplicationBuilder<'a, 'b, T>,
    ) -> Result<ApplicationBuilder<'a, 'b, T>> {
        Ok(
            builder
                .with_resource(Time::default())
                .register::<Position>()
                .register::<Velocity>()
                .with_resource(FPSCounter::new(10))
                .with(ApplyVelocity, "game_physics", &[])
                .with(UpdateTransform, "game_update_transform", &[])
                .with(CollideSystem, "game_collide", &[])
                .with(FPSCounterSystem, "fps", &["game_physics", "game_update_transform", "game_collide"])
                .with(FPSOutput, "fps_output", &["fps"])
            ,

        )
    }
}