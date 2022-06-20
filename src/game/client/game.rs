use std::{rc::Rc, cell::Cell};

use anyhow::Result;
use winit::window::Window;

use crate::state::State;

use super::screen::{world_screen::WorldScreen, menu_screen::MenuScreen};

pub struct Game {
    pub show_menu : Rc<Cell<bool>>,
    pub state     : State,
}

impl Game {
    pub async fn new(window: &Window) -> Result<Self> {
        let mut game = Self {
            show_menu: Rc::new(Cell::new(false)),
            state: State::new(window).await?
        };

        game.state.screen_stack.push(Box::new(WorldScreen::new(game.state.device.clone(), game.state.queue.clone(), &game.state.config, game.show_menu.clone())?));
        game.state.screen_stack.push(Box::new(MenuScreen::new(game.state.device.clone(), &game.state.config, game.show_menu.clone())?));

        return Ok(game);
    }
}