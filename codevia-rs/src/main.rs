use ratatui::layout::{Constraint, Direction, Layout};

use crate::ui::components::header::render_header;

mod ui;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(frame.area());

                render_header(main_layout[0], frame);
            })?;
        }
    })
}
