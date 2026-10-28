use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::ui::{Component, PromptInputState, UiState, styles::colors::Colors};

pub fn render_prompt_box(
    area: Rect,
    frame: &mut Frame,
    prompt_input_state: &PromptInputState,
    ui_state: &UiState,
) {
    let prompt_box = Block::new()
        .title(" Prompt Box ")
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg({
            let (r, g, b) = Colors::Border.rgba();
            Color::Rgb(r, g, b)
        }))
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    let inner_area = prompt_box.inner(area);

    let line =
        Span::from(prompt_input_state.input.as_str()).style(Style::default().fg(Color::White));

    let cursor = Span::from("▁")
        .style(Style::default().fg(Color::White).slow_blink())
        .add_modifier(Modifier::BOLD);

    let cursor = if ui_state.is_current_component_focused(&Component::PromptBox) {
        cursor
    } else {
        cursor.hidden()
    };

    let text = Paragraph::new(Line::from(vec![line, cursor]));

    frame.render_widget(prompt_box, area);
    frame.render_widget(text, inner_area);
}
