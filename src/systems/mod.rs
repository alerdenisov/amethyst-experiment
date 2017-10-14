use {ARENA_HEIGHT, ARENA_WIDTH};
use components::{Velocity,Position};
use amethyst::ecs::transform::{LocalTransform};
use amethyst::timing::Time;
use specs::{ReadStorage, System, WriteStorage, Fetch, ParJoin};
use rayon::iter::ParallelIterator;
use amethyst::ecs::util::resources::FPSCounter;

pub struct ApplyVelocity;

impl<'a> System<'a> for ApplyVelocity {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        Fetch<'a, Time>,
    );

    fn run(&mut self, (mut pos, vel, time) : Self::SystemData) {
        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        (&mut pos, &vel).par_join().for_each(|(pos, vel)| {
            pos.0 += vel.0 * delta_time;
            pos.1 += vel.1 * delta_time;
        });
    }
}

pub struct UpdateTransform;

impl<'a> System<'a> for UpdateTransform {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, LocalTransform>
    );

    fn run(&mut self, (pos, mut transform) : Self::SystemData) {
        (&pos, &mut transform).par_join().for_each(|(pos, transform)| {
            transform.translation[0] = pos.0;
            transform.translation[1] = pos.1;
        });

    }
}

pub struct CollideSystem;

impl<'a> System<'a> for CollideSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Position>
    );

    fn run(&mut self, (mut vel, pos) : Self::SystemData) {
        (&mut vel, &pos).par_join().for_each(|(vel, pos)| {
            if(pos.0.abs() > ARENA_WIDTH / 2.0) {
                vel.0 = -vel.0;
            }

            if(pos.1.abs() > ARENA_HEIGHT / 2.0) {
                vel.1 = -vel.1;
            }
        })
    }
}

pub struct FPSOutput;

impl<'a> System<'a> for FPSOutput {
    type SystemData = Fetch<'a, FPSCounter>;

    fn run(&mut self, fps : Self::SystemData) {
        println!("FPS: {}", fps.sampled_fps());
    }
}