use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::ui::styles::colors::Colors;

pub fn render_conversations(area: Rect, frame: &mut Frame) {
    let conversations = Block::new()
        .title(" Conversations ")
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg({
            let (r, g, b) = Colors::Border.rgba();
            Color::Rgb(r, g, b)
        }))
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    let content =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quisquam, quod.".repeat(100);

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, conversations.inner(area));
    frame.render_widget(conversations, area);
}
