use {ARENA_HEIGHT, ARENA_WIDTH};
use components::{Position, Velocity};

use rand;
use rand::Rng;

use amethyst::assets::{AssetFuture, BoxedErr};
use amethyst::ecs::World;
use amethyst::ecs::rendering::{Factory, MaterialComponent, MeshComponent};
use amethyst::ecs::transform::{LocalTransform, Transform};
use amethyst::prelude::*;
use amethyst::renderer::prelude::*;
use futures::prelude::*;


pub struct Game;

impl State for Game {
    fn on_start(&mut self, engine: &mut Engine) {
        // Setup our game.
        initialise_entites(&mut engine.world);
        initialise_camera(&mut engine.world);
        hide_cursor(&mut engine.world);
    }
}

fn load_asset<T, F>(world: &mut World, f: F) -> AssetFuture<T::Item>
    where
        T: IntoFuture<Error = BoxedErr>,
        T::Future: 'static,
        F: FnOnce(&mut World) -> T,
{
    let future = f(world).into_future();
    let future: Box<Future<Item = T::Item, Error = BoxedErr>> = Box::new(future);
    AssetFuture(future.shared())
}

fn initialise_camera(world: &mut World) {
    world.add_resource(Camera {
        eye: [0.0, 0.0, 1.0].into(),
        // Make the arena fit perfectly in the view of the camera.
        proj: Projection::orthographic(-ARENA_WIDTH / 2.0, ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, -ARENA_HEIGHT / 2.0).into(),
        forward: [0., 0., -1.0].into(),
        right: [1.0, 0.0, 0.0].into(),
        up: [0., 1.0, 0.].into(),
    });
}

/// Hide the cursor so it's invisible while playing.
fn hide_cursor(world: &mut World) {
    use amethyst::ecs::rendering::resources::WindowMessages;
    use amethyst::winit::CursorState;

    world
        .write_resource::<WindowMessages>()
        .send_command(|win| {
            if let Err(err) = win.set_cursor_state(CursorState::Hide) {
                eprintln!("Unable to make cursor hidden! Error: {:?}", err);
            }
        });
}

fn initialise_entites(world: &mut World) {
    // Create the mesh and the material needed.
    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(-1.0, -1.0, 1.0, 1.0),
    );

    let material = create_colour_material(world,  [0.0, 0.0, 1.0, 1.0]);

    for _ in 1..500 {
        let vel_x : f32 = rand::thread_rng().gen_range(70.0, 120.0);
        let vel_y : f32 = rand::thread_rng().gen_range(70.0, 120.0);
        world
            .create_entity()
            .with(mesh.clone())
            .with(material.clone())
            .with(Transform::default())
            .with(LocalTransform::default())
            .with(Position(0.0, 0.0))
            .with(Velocity(vel_x, vel_y))
            .build();
    }
}


/// Converts a vector of vertices into a mesh.
fn create_mesh(world: &mut World, vertices: Vec<PosTex>) -> AssetFuture<MeshComponent> {
    let mesh = Mesh::build(vertices);

    load_asset(world, move |world| {
        let factory = world.read_resource::<Factory>();
        factory
            .create_mesh(mesh)
            .map(MeshComponent::new)
            .map_err(BoxedErr::new)
    })
}

/// Creates a solid material of the specified colour.
fn create_colour_material(world: &mut World, colour: [f32; 4]) -> AssetFuture<MaterialComponent> {
    let texture = Texture::from_color_val(colour);
    let material = MaterialBuilder::new().with_albedo(texture);

    load_asset(world, move |world| {
        let factory = world.read_resource::<Factory>();
        factory
            .create_material(material)
            .map(MaterialComponent)
            .map_err(BoxedErr::new)
    })
}
/// Generates six vertices forming a rectangle.
fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: [left, bottom, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [right, top, 0.],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
    ]
}