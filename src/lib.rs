pub mod state;
pub mod game;
pub mod graphics;
pub mod screen;
pub mod utils;

use game::client::game::Game;

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
    utils::init_logger();
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    window.set_cursor_grab(true).unwrap();

    let mut game = Game::new(&window).await.unwrap();
    let mut focused = false;
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                game.state.update(now);
                game.state.render();
            }

            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta, }, .. }
                => { if focused { game.state.mouse(delta) } }

            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::Focused(is_focused) => { focused = is_focused; }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    
                    WindowEvent::Resized            ( physical_size )      => game.state.resize(physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => game.state.resize(*new_inner_size),

                    _ => if focused { game.state.input(&event) },
                }
            }

            _ => {}
        }
    });
}

// Helpers
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