extern crate specs;
extern crate rayon;
extern crate rand;
#[macro_use]
extern crate specs_derive;
extern crate amethyst;
extern crate futures;

mod components;
mod systems;
mod bundle;
mod state;


use std::time::Duration;

use amethyst::Result;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::ecs::audio::DjBundle;
use amethyst::ecs::input::InputBundle;
use amethyst::ecs::rendering::{MaterialComponent, MeshComponent, RenderBundle};
use amethyst::ecs::transform::{Transform, TransformBundle};
use amethyst::prelude::*;
use amethyst::renderer::Config as DisplayConfig;
use amethyst::renderer::prelude::*;
use amethyst::util::frame_limiter::FrameRateLimitStrategy;

const ARENA_HEIGHT: f32 = 600.0;
const ARENA_WIDTH: f32 = 1000.0;


fn main() {
    if let Err(e) = run() {
        println ! ("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}

type DrawFlat = pass::DrawFlat<PosTex, MeshComponent, MaterialComponent, Transform>;

fn run() -> Result<()> {
    use state::Game;
    use bundle::GameBundle;


    let display_config_path = format!(
        "{}/resources/display.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let game = Application::build(Game)?
//        .with_frame_limit(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)), 144)
//        .with_bundle(
//            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path),
//        )?
        .with_bundle(GameBundle)?
        .with_bundle(
            TransformBundle::new().with_dep(&["game_physics", "game_update_transform", "game_collide"]))?
        .with_bundle(
            RenderBundle::new(
                Pipeline::build().with_stage(
                    Stage::with_backbuffer()
                        .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                        .with_pass(DrawFlat::new()),
                ),
            ).with_config(DisplayConfig::load(display_config_path)),
        )?;
    Ok(game.build()?.run())
}