use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

use crate::ui::{UiState, colors::Colors};

static KEYS_ARR: &[(&str, &str)] = &[
    ("Esc", "Interrupt"),
    ("Ctrl + c", "Exit"),
    ("Shift + Tab", "Change focus"),
    ("/", "Commands"),
];

pub fn render_key_shortcuts(area: Rect, frame: &mut Frame, _ui_state: &UiState) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);

    let spec = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, KEYS_ARR.len() as u32); KEYS_ARR.len()],
    )
    .split(inner_area);

    for (i, (key, description)) in KEYS_ARR.iter().enumerate() {
        let key_label = styled_key_label(key, description);
        frame.render_widget(key_label, spec[i]);
    }
}

fn styled_key_label<'a>(key: &'a str, description: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(
            format!("[{}]: ", key),
            Style::default().fg({
                let (r, g, b) = Colors::MutedForeground.rgba();
                Color::Rgb(r, g, b)
            }),
        ),
        Span::styled(
            description,
            Style::default().fg({
                let (r, g, b) = Colors::Primary.rgba();
                Color::Rgb(r, g, b)
            }),
        ),
    ])
    .alignment(Alignment::Center)
}
