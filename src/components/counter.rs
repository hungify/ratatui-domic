use color_eyre::Result;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{block::Position, Block, Paragraph},
    Frame,
};

use super::Component;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub struct Counter {
    state: Option<AppState>,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Component for Counter {
    fn register_state_handler(&mut self, state: AppState) -> Result<()> {
        self.state = Some(state);
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let Some(state) = &self.state else {
            return Ok(());
        };

        let amount = state.get_amount()?;
        let increase_by_interpolation = " Increment by {amount}".replace("{amount}", &amount);
        let decrease_by_interpolation = " Decrement by {amount}".replace("{amount}", &amount);

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

        let count = state.get_count()?;
        let count_text = Text::from(vec![Line::from(vec![
            "Count: ".into(),
            count.to_string().yellow(),
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
        let amount_paragraph = Paragraph::new(amount.as_str())
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
}
