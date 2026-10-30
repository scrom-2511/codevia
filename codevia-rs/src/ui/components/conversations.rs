use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::ui::{ChatMessage, ConversationState, Role, ToolType};

enum ItemData<'a> {
    ToolCall {
        block: Block<'a>,
        lines: Vec<Line<'a>>,
        height: u16,
    },
    TextMessage {
        lines: Vec<Line<'a>>,
        height: u16,
    },
}

pub fn render_conversations(area: Rect, frame: &mut Frame) {
    let conversation_state = ConversationState {
        conversation: vec![
            ChatMessage {
                role: Role::User,
                message: "Hello! How are you doing today?",
            },
            ChatMessage {
                role: Role::Model,
                message: "Hi! I'm doing great, thanks for asking. How can I help you today?",
            },
            ChatMessage {
                role: Role::User,
                message: "I'm learning Rust and trying to build a terminal-based chat application using Ratatui.",
            },
            ChatMessage {
                role: Role::ToolCall(ToolType::Read, vec!["file.txt"]),
                message: "",
            },
            ChatMessage {
                role: Role::Model,
                message: "That's awesome! Ratatui is a great choice for building terminal user interfaces in Rust.\n\nYou can use Paragraph, Block, Layout, and other widgets to build your chat interface.",
            },
            ChatMessage {
                role: Role::ToolCall(ToolType::Write, vec!["file2.txt"]),
                message: "",
            },
            ChatMessage {
                role: Role::User,
                message: "How should I structure my application?",
            },
            ChatMessage {
                role: Role::Model,
                message: "I recommend separating your application into three main parts:\n\n1. Application state - Stores messages, input text, and UI state.\n2. Event handling - Handles keyboard input and user interactions.\n3. Rendering - Draws the interface using Ratatui widgets.",
            },
            ChatMessage {
                role: Role::User,
                message: "Can you show me a simple Rust function for adding messages?",
            },
            ChatMessage {
                role: Role::Model,
                message: "Sure! You can create a helper method like this:\n\nfn add_message(&mut self, role: Role, message: String) {\n    self.conversation.push(ChatMessage {\n        role,\n        message,\n    });\n}",
            },
            ChatMessage {
                role: Role::User,
                message: "Perfect! Next, I want to add scrolling.",
            },
            ChatMessage {
                role: Role::Model,
                message: "You can implement scrolling using a vertical Scrollbar or by tracking a scroll offset in your application state.",
            },
            ChatMessage {
                role: Role::User,
                message: "How should I structure my application?",
            },
            ChatMessage {
                role: Role::Model,
                message: "I recommend separating your application into three main parts:\n\n1. Application state - Stores messages, input text, and UI state.\n2. Event handling - Handles keyboard input and user interactions.\n3. Rendering - Draws the interface using Ratatui widgets.",
            },
        ],
    };

    let usable_width = area.width;

    let mut items: Vec<ItemData> = Vec::with_capacity(conversation_state.conversation.len());

    for chat_message in &conversation_state.conversation {
        match &chat_message.role {
            Role::ToolCall(tool_type, files) => {
                let border_color = match tool_type {
                    ToolType::Read => Color::Yellow,
                    ToolType::Write => Color::Magenta,
                    ToolType::Search => Color::Blue,
                };

                let mut inner_lines: Vec<Line> = Vec::new();

                for file in files {
                    inner_lines.push(Line::from(vec![
                        Span::styled(" - ", Style::default().fg(border_color)),
                        Span::styled(
                            *file,
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]));
                }

                let block_title = Line::from(vec![Span::styled(
                    format!(" {:?} ", tool_type),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::BOLD),
                )]);

                let tool_block = Block::default()
                    .title(block_title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(border_color))
                    .padding(Padding::horizontal(1));

                let content_width = usable_width.saturating_sub(2).max(1);
                let visual_lines = estimate_visual_lines(&inner_lines, content_width);
                let needed_height = visual_lines.saturating_add(2); // +2 for top & bottom borders

                items.push(ItemData::ToolCall {
                    block: tool_block,
                    lines: inner_lines,
                    height: needed_height,
                });
            }
            _ => {
                let mut lines: Vec<Line> = Vec::new();

                let role_span = if chat_message.role == Role::User {
                    Span::styled(
                        "You:",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled(
                        "Assistant:",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                };

                lines.push(Line::from(vec![
                    Span::styled(
                        "❯ ",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    role_span,
                ]));

                lines.push(Line::from(chat_message.message));

                let content_width = usable_width.max(1);
                let visual_lines = estimate_visual_lines(&lines, content_width);

                items.push(ItemData::TextMessage {
                    lines,
                    height: visual_lines,
                });
            }
        }
    }

    // The height from which we start rendering items
    let mut start_height_for_rendering_items: u16 = area.y;

    // The maximum height until which we can render items
    let max_height_for_rendering_items = area.y.saturating_add(area.height);

    for item in items {
        let curr_item_height = match item {
            ItemData::ToolCall { height, .. } => height,
            ItemData::TextMessage { height, .. } => height,
        };

        let widget_area = Rect {
            x: area.x,
            y: start_height_for_rendering_items,
            width: usable_width,
            height: curr_item_height,
        };

        if widget_area.y < max_height_for_rendering_items {
            match item {
                ItemData::ToolCall { block, lines, .. } => {
                    let paragraph = Paragraph::new(lines)
                        .block(block)
                        .wrap(Wrap { trim: false });
                    frame.render_widget(paragraph, widget_area);
                }
                ItemData::TextMessage { lines, .. } => {
                    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
                    frame.render_widget(paragraph, widget_area);
                }
            }
        }

        start_height_for_rendering_items += curr_item_height + 1;
    }
}

fn estimate_visual_lines(lines: &[Line], width: u16) -> u16 {
    let inner_width = width.max(1);
    let mut total: u16 = 0;

    for line in lines {
        let line_len: usize = line.spans.iter().map(|s| s.content.chars().count()).sum();
        let rows = if line_len == 0 {
            1
        } else {
            ((line_len as u16).saturating_add(inner_width.saturating_sub(1))) / inner_width
        };
        total = total.saturating_add(rows.max(1));
    }
    total
}
