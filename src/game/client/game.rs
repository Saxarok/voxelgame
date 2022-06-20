use anyhow::Result;
use winit::window::Window;

use crate::state::State;

use super::screen::{world_screen::WorldScreen, menu_screen::MenuScreen};

pub struct Game {
    pub state: State,
}

impl Game {
    pub async fn new(window: &Window) -> Result<Self> {
        let mut state = State::new(window).await?;

        state.screen_stack.push(Box::new(WorldScreen::new(state.device.clone(), state.queue.clone(), &state.config)?));
        state.screen_stack.push(Box::new(MenuScreen::new(state.device.clone(), &state.config)?));

        return Ok(Self {
            state
        });
    }
}