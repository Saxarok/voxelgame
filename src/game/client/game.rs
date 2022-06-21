use anyhow::Result;
use winit::{window::Window, event::WindowEvent};

use crate::state::State;

use super::screen::{world_screen::WorldScreen, menu_screen::MenuScreen};

pub struct Game {
    state: State,
}

impl Game {
    pub async fn new(window: &Window) -> Result<Self> {
        let mut game = Self {
            state: State::new(window).await?
        };

        game.state.screen_stack.push(Box::new(WorldScreen::new(game.state.device.clone(), game.state.queue.clone(), &game.state.config)?));
        game.state.screen_stack.push(Box::new(MenuScreen::new(&game.state.device, &game.state.config)?));

        return Ok(game);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.state.resize(new_size); }
    pub fn mouse(&mut self, delta: (f64, f64)) { self.state.mouse(delta); }
    pub fn input(&mut self, event: &WindowEvent) { self.state.input(event); }
    pub fn update(&mut self, now: instant::Instant) { self.state.update(now); }
    pub fn render(&mut self) { self.state.render(); }
}