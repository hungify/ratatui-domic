use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{block::Position, Block, Paragraph},
    Frame,
};

use super::Component;
use crate::action::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    count: i32,
    amount: String,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Self {
        Self {
            count: 0,
            amount: String::from("1"),
        }
    }
}

impl Component for Counter {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let increase_by_interpolation = " Increment by {amount}".replace("{amount}", &self.amount);
        let decrease_by_interpolation = " Decrement by {amount}".replace("{amount}", &self.amount);

        let instructions = Line::from(vec![
            " Increment amount".into(),
            " <Up> |".blue().bold(),
            " Decrement amount".into(),
            " <Down> |".blue().bold(),
            increase_by_interpolation.into(),
            " <Left> |".blue().bold(),
            decrease_by_interpolation.into(),
            " <Right> |".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let title = Line::from(" Counter App ".bold());
        let container_block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let container_paragraph = Paragraph::new(Text::default()).block(container_block.clone());
        frame.render_widget(container_paragraph, area);

        let count_text = Text::from(vec![Line::from(vec![
            "Count: ".into(),
            self.count.to_string().yellow(),
        ])]);
        let count_block = Block::default()
            .title_alignment(Alignment::Center)
            .title_position(Position::Bottom);

        let count_paragraph = Paragraph::new(count_text)
            .alignment(Alignment::Center) // Center the count text
            .block(count_block);

        let amount_block = Block::bordered()
            .title("Amount")
            .title_alignment(Alignment::Center);
        let amount_paragraph = Paragraph::new(self.amount.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(amount_block);

        let inner_area = container_block.inner(area);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let chunks_inner_area: std::rc::Rc<[Rect]> = horizontal.split(inner_area);

        let vertical_left = Layout::vertical([Constraint::Length(3)]);
        let vertical_right = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Length(10),
            Constraint::Percentage(50),
        ]);
        let chunks_amount = vertical_left.split(chunks_inner_area[0]);
        let chunks_count = vertical_right.split(chunks_inner_area[1]);

        frame.render_widget(amount_paragraph, chunks_amount[0]);
        frame.render_widget(count_paragraph, chunks_count[1]);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Render => {}
            Action::IncrementBy(n) => self.count = self.count.saturating_add(n),
            Action::DecrementBy(n) => self.count = self.count.saturating_sub(n),
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let amount_number = self.amount.parse::<i32>().unwrap_or(1);
        match key.code {
            KeyCode::Left => Ok(Some(Action::DecrementBy(amount_number))),
            KeyCode::Right => Ok(Some(Action::IncrementBy(amount_number))),
            KeyCode::Up => {
                let value = self.amount.parse::<i32>().unwrap_or(1);
                self.amount = value.saturating_add(1).to_string();
                Ok(None)
            }
            KeyCode::Down => {
                let value = self.amount.parse::<i32>().unwrap_or(1);
                self.amount = if value > 1 {
                    value.saturating_sub(1).to_string()
                } else {
                    value.to_string()
                };
                Ok(None)
            }

            _ => Ok(None),
        }
    }
}
