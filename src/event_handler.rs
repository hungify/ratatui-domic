use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    action::Action,
    event::Event,
    state::AppState,
};

pub struct EventHandler {
    state: AppState,
}

impl EventHandler {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn handle(&self, event: Event) -> Result<Option<Action>> {
        match event {
            Event::Key(key) => self.handle_key_event(key),
            Event::Mouse(_mouse) => Ok(None),
            Event::Resize(width, height) => Ok(Some(Action::Resize(width, height))),
            Event::Tick => Ok(Some(Action::Tick)),
            Event::Render => Ok(Some(Action::Render)),
            _ => Ok(None),
        }
    }

    fn handle_key_event(&self, key: KeyEvent) -> Result<Option<Action>> {
        // Global key handlers that work regardless of context
        match key.code {
            KeyCode::Char('q') if key.modifiers.is_empty() => {
                return Ok(Some(Action::Quit));
            }
            KeyCode::Char('h') if key.modifiers.is_empty() => {
                return Ok(Some(Action::Help));
            }
            // Counter controls
            KeyCode::Left => {
                self.state.decrement_count()?;
                return Ok(Some(Action::Render));
            }
            KeyCode::Right => {
                self.state.increment_count()?;
                return Ok(Some(Action::Render));
            }
            KeyCode::Up => {
                self.state.increment_amount()?;
                return Ok(Some(Action::Render));
            }
            KeyCode::Down => {
                self.state.decrement_amount()?;
                return Ok(Some(Action::Render));
            }
            KeyCode::Esc => {
                return Ok(Some(Action::Quit));
            }
            _ => {}
        }

        // Handle CTRL+key combinations
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => {
                    return Ok(Some(Action::Quit));
                }
                KeyCode::Char('r') => {
                    return Ok(Some(Action::Refresh));
                }
                _ => {}
            }
        }

        Ok(None)
    }
}
