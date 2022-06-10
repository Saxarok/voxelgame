mod state;
mod game;
mod graphics;
mod screen;

use game::game::Game;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::LogicalSize,
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

const WIDTH  : u32 = 1200;
const HEIGHT : u32 = 800;

// Entrypoint
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    init_logger();
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);

    let mut game = Game::new(&window).await.unwrap();
    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow|
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;

                game.state.update(dt);
                game.state.render();
            }

            Event::DeviceEvent { event: DeviceEvent::MouseMotion{ delta, }, .. }
                => { game.state.mouse(delta) }

            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    
                    WindowEvent::Resized            ( physical_size )      => game.state.resize(physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => game.state.resize(*new_inner_size),

                    _ => game.state.input(&event),
                }
            }

            _ => {}
        }
    );
}

// Helpers
fn init_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else { env_logger::init(); }
    }
}

fn create_window(event_loop: &EventLoop<()>) -> Window {
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")] {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(WIDTH, HEIGHT));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("body")?;
                let canvas = web_sys::Element::from(window.canvas());

                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append canvas to document body.");
    }

    return window;
}