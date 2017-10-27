extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate rand;
extern crate time;

extern crate caper;

use std::os::raw::c_void;
use std::ptr;
use std::slice;
use std::str;

use glutin::{ContextBuilder, Event, EventsLoop, GlContext, GlWindow, WindowBuilder, WindowEvent};

extern "system" fn log(_source: gl::types::GLenum,
                       _type: gl::types::GLenum,
                       _id: gl::types::GLuint,
                       _severity: gl::types::GLenum,
                       length: gl::types::GLsizei,
                       message: *const gl::types::GLchar,
                       _user_param: *mut c_void) {
    let slice = unsafe { slice::from_raw_parts(message as *const u8, length as usize) };
    let string = str::from_utf8(slice);
    println!("{:?}", string);
}

fn main() {
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Caper")
        .with_dimensions(768, 768);
    let context_builder = ContextBuilder::new()
        .with_vsync(true);
    let window = GlWindow::new(window_builder, context_builder, &events_loop)
        .unwrap();

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const c_void);
        window.make_current().unwrap();
        gl::Enable(gl::DEPTH_TEST);
    }

    unsafe {
        gl::DebugMessageCallback(log, ptr::null());
    }

    let projection_transform = cgmath::perspective(
        cgmath::Deg(45.0),
        768.0 / 768.0,
        0.1,
        100.0,
    );

    let mut rng = rand::StdRng::new().unwrap();

    let mut world = caper::world::World::new(cgmath::Vector2::new(0.0, 0.0));
    world.map.sectors.insert(cgmath::Vector2::new( 0, -0), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new( 0, -1), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new( 0, -2), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new( 0,  0), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new( 0,  1), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new( 0,  2), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new(-1, -0), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new(-1, -1), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new(-1, -2), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new(-1,  0), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new(-1,  1), caper::world::map::Sector::new());
    world.map.sectors.insert(cgmath::Vector2::new(-1,  2), caper::world::map::Sector::new());
    world.spiders.spawn(&mut rng, cgmath::Vector2::new( 0.0,  0.0));
    world.spiders.spawn(&mut rng, cgmath::Vector2::new( 1.0,  1.0));
    world.spiders.spawn(&mut rng, cgmath::Vector2::new( 2.0,  2.0));
    world.spiders.spawn(&mut rng, cgmath::Vector2::new( 3.0,  3.0));

    let mut simulation_state = caper::world::simulation::SimulationState::new();
    let graphics_state = caper::world::graphics::GraphicsState::new();

    let mut previous_simulation = time::precise_time_ns();
    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent{event: WindowEvent::Closed, ..} => {
                    println!("{:?}", event);
                    running = false;
                },
                Event::WindowEvent{event: WindowEvent::Resized(w, h), ..} => {
                    println!("{:?}", event);
                    window.resize(w, h);
                    unsafe {
                        gl::Viewport(0 as gl::types::GLint,
                                     0 as gl::types::GLint,
                                     w as gl::types::GLsizei,
                                     h as gl::types::GLsizei);
                    }
                },
                _ => (),
            }
        });

        let current_simulation = time::precise_time_ns();
        let dt = (current_simulation - previous_simulation) as f32 / 1000000.0;
        previous_simulation = current_simulation;
        simulation_state.simulate(&mut rng, dt, &mut world);

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
        graphics_state.draw(projection_transform, &world);
        window.swap_buffers().unwrap();
    }
}
