use ratatui::{
    Frame,
    prelude::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::styles::colors::Colors;

pub fn render_header(area: Rect, frame: &mut Frame) -> () {
    let header = Paragraph::new(" Codevia | Coding agent ")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg({
                    let (r, g, b) = Colors::Border.rgba();
                    Color::Rgb(r, g, b)
                })),
        );

    frame.render_widget(header, area);
}
