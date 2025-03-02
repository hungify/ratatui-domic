use color_eyre::Result;
use ratatui::{
    layout::Alignment,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

use super::Component;

#[derive(Debug, Clone)]
pub struct Home {
    state: Option<AppState>,
}

impl Home {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Component for Home {
    fn register_state_handler(&mut self, state: AppState) -> Result<()> {
        self.state = Some(state);
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let Some(state) = &self.state else {
            return Ok(());
        };

        let count = state.get_count()?;
        let amount = state.get_amount()?;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = Text::from(vec![
            Line::from(vec!["Welcome to ".into(), "Ratatui Domic".blue().bold()]),
            Line::from(""),
            Line::from(vec![
                "Current counter: ".into(),
                count.to_string().yellow().bold(),
            ]),
            Line::from(vec![
                "Current increment amount: ".into(),
                amount.yellow().bold(),
            ]),
        ]);

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
        Ok(())
    }
}
