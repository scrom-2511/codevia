use std::env;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{State, ui::styles::colors::Colors};

pub fn render_header(area: Rect, frame: &mut Frame, state: &State) -> () {
    let block = Block::default()
        .title(" Codevia ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg({
            let (r, g, b) = Colors::Border.rgba();
            Color::Rgb(r, g, b)
        }))
        .padding(Padding::horizontal(2));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let [text_area] = Layout::vertical([Constraint::Length(1)])
        .flex(Flex::Center)
        .areas(inner_area);

    let [project_area, tokens_area] =
        Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
            .flex(Flex::Center)
            .areas(text_area);

    let cwd = env::current_dir().unwrap().display().to_string();

    let project_name = Paragraph::new(styled_heading("Project: ", &cwd))
        .style(Style::default().add_modifier(Modifier::BOLD));

    let tokens_used = Paragraph::new(styled_heading("Tokens Used: ", "234234234"))
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::HorizontalAlignment::Right);

    frame.render_widget(project_name, project_area);
    frame.render_widget(tokens_used, tokens_area);
}

pub fn styled_heading<'a>(label: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(
            label,
            Style::default().fg({
                let (r, g, b) = Colors::MutedForeground.rgba();
                Color::Rgb(r, g, b)
            }),
        ),
        Span::styled(
            value,
            Style::default().fg({
                let (r, g, b) = Colors::Primary.rgba();
                Color::Rgb(r, g, b)
            }),
        ),
    ])
}
