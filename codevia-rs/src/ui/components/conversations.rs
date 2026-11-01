use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
};

use crate::ui::{
    ChatMessage, Component, ConversationState, Role, ToolType, UiState, colors::Colors,
};

pub fn render_conversations(
    area: Rect,
    frame: &mut Frame,
    ui_state: &mut UiState,
    conversation_state: &ConversationState,
) {
    let mut items: Vec<Line> = Vec::with_capacity(conversation_state.conversation.len());

    for chat_message in &conversation_state.conversation {
        match &chat_message.role {
            Role::ToolCall(tool_type, files) => {
                for file in files {
                    items.push(Line::from("    │"));

                    items.push(Line::from(vec![
                        Span::from("    ╰──  "),
                        Span::from(format!("{} ({})", tool_type.as_str(), *file)),
                    ]));
                }
            }
            _ => {
                if chat_message.role == Role::User {
                    items.push(
                        Line::from(vec![
                            Span::styled("❯ ", Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled(
                                chat_message.message,
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            Span::from(" "),
                        ])
                        .style(Style::default().bg({
                            let (r, g, b) = Colors::Card.rgb();
                            Color::Rgb(r, g, b)
                        })),
                    );
                } else {
                    items.push(Line::from(vec![
                        Span::styled(
                            "◆ ",
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::from(chat_message.message),
                    ]));
                };
            }
        }

        items.push(Line::from(""));
    }

    let text = Text::from(items);

    let max_scroll_offset = (text.height() as u16).saturating_sub(area.height);
    ui_state
        .scroll
        .set_max_offset(&Component::Conversations, max_scroll_offset as i16);
    let scroll_offset =
        (ui_state.scroll.get_offset(&Component::Conversations) as u16).min(max_scroll_offset);

    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));

    frame.render_widget(paragraph, area);
}
