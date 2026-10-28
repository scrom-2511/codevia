use std::io::Error;

use crossterm::event::{Event, KeyCode, KeyModifiers, read};

use crate::{
    State,
    ui::{Component, PromptInputState},
};

pub fn keyboard_input_handler(
    state: &mut State,
    prompt_input_state: &mut PromptInputState,
) -> Result<bool, Error> {
    if state.component_focus.get_current_focused_component() == Component::PromptBox {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(false);
                }
                KeyCode::Char(c) => prompt_input_state.append(c),
                KeyCode::Backspace => prompt_input_state.pop(),
                KeyCode::Delete => prompt_input_state.clear(),
                _ => {}
            }
        }
    } else {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Up => {
                    state.scroll.increment_offset();
                }
                KeyCode::Down => {
                    state.scroll.decrement_offset();
                }
                KeyCode::BackTab => {
                    state.component_focus.change_focus();
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(false);
                }
                _ => {}
            }
        }
    }

    Ok(true)
}
