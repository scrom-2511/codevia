use std::io::Error;

use crossterm::event::{Event, KeyCode, read};

use crate::State;

pub fn keyboard_input_handler(state: &mut State) -> Result<(), Error> {
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
            _ => {}
        }
    }

    Ok(())
}
