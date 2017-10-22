extern crate cgmath;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate rand;

extern crate caper;

use cgmath::{Deg, Vector2};
use gfx::Device;
use glutin::{ContextBuilder, Event, EventsLoop, GlContext, WindowBuilder, WindowEvent};
use rand::StdRng;

use caper::world;

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

fn main() {
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Caper")
        .with_dimensions(768, 768);
    let context_builder = ContextBuilder::new()
        .with_vsync(true);
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut rng = StdRng::new().unwrap();
    let mut world = world::World::new(Vector2::new(0.0, 0.0));
    world.entities.spiders.spawn(&mut rng, Vector2::new(-0.5, -0.5));
    world.entities.spiders.spawn(&mut rng, Vector2::new( 0.5,  0.5));

    let mut world_draw_state = world::graphics::DrawState::new(&mut factory, main_color.clone(), main_depth.clone()).unwrap();

    let projection_transform = cgmath::perspective(
        Deg(45.0),
        768.0 / 768.0,
        0.1,
        100.0,
    );

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent{event: WindowEvent::Closed, ..} => {
                    println!("{:?}", event);
                    running = false;
                },
                Event::WindowEvent{event: WindowEvent::Resized(_, _), ..} => {
                    println!("{:?}", event);
                    gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth);
                    world_draw_state = world::graphics::DrawState::new(&mut factory, main_color.clone(), main_depth.clone()).unwrap();
                },
                Event::WindowEvent{event: WindowEvent::KeyboardInput{input, ..}, ..} => {
                    let player_movement = match input.scancode {
                        0x11 => Vector2::new( 0.0,  1.0), // W
                        0x1F => Vector2::new( 0.0, -1.0), // S
                        0x1E => Vector2::new(-1.0,  0.0), // A
                        0x20 => Vector2::new( 1.0,  0.0), // D
                        _ => Vector2::new(0.0, 0.0),
                    };
                    world.entities.player.position += player_movement / 10.0;
                },
                _ => (),
            }
        });

        world.simulate(&mut rng);

        encoder.clear(&main_color, [1.0, 1.0, 1.0, 1.0]);
        encoder.clear_depth(&main_depth, 1.0);
        world_draw_state.draw(&mut encoder, projection_transform, &world).unwrap();
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
