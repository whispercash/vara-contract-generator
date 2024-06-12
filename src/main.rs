mod cli;
use cli::Cli;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::read_args();
    Ok(())
}

// use std::io::{self, stdout};

// use crossterm::{
//     event::{self, Event, KeyCode},
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//     ExecutableCommand,
// };
// use ratatui::{prelude::*, widgets::*};

// fn main() -> io::Result<()> {
//     enable_raw_mode()?;
//     stdout().execute(EnterAlternateScreen)?;
//     let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

//     let mut should_quit = false;
//     while !should_quit {
//         terminal.draw(ui)?;
//         should_quit = handle_events()?;
//     }

//     disable_raw_mode()?;
//     stdout().execute(LeaveAlternateScreen)?;
//     Ok(())
// }

// fn handle_events() -> io::Result<bool> {
//     if event::poll(std::time::Duration::from_millis(50))? {
//         if let Event::Key(key) = event::read()? {
//             if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                 return Ok(true);
//             }
//         }
//     }
//     Ok(false)
// }

// fn ui(frame: &mut Frame) {
//     let main_layout = Layout::new(
//         Direction::Vertical,
//         [
//             Constraint::Length(1), // from top for 1st element
//             Constraint::Max(10), // min length for second element
//             Constraint::Length(1), // from bottom for third element
//         ],
//     )
//     .split(frame.size());

//     // main_layout[0] -- top portion
//     frame.render_widget(
//         Block::new().borders(Borders::TOP).title("Title Bar"),
//         main_layout[0],
//     );

//     // main_layout[2] -- bottom portion
//     frame.render_widget(
//         Block::new().borders(Borders::TOP).title("Status Bar"),
//         main_layout[2],
//     );

//     // main_layout[1] -- middle portion. create another layout
//     let inner_layout = Layout::new(
//         Direction::Horizontal,
//         [
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25)
//         ],
//     )
//     .split(main_layout[1]);

//     let span1 = Span::raw("Hello Sushmit ");

//     // let first_layout = frame.render_widget(Block::bordered().title("Left"), inner_layout[0]);
//     frame.render_widget(Block::bordered().title("Right"), inner_layout[1]);
//     frame.render_widget(Block::bordered().title("Right"), inner_layout[2]);
//     frame.render_widget(Block::bordered().title("Right"), inner_layout[3]);

//     let sub_layout = Layout::new(
//         Direction::Vertical,
//         [
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25),
//             Constraint::Percentage(25)
//         ],
//     )
//     .split(inner_layout[0]);

//     frame.render_widget(Paragraph::new(span1), sub_layout[0]);
//     frame.render_widget(Paragraph::new(span1), sub_layout[2]);
//     frame.render_widget(Paragraph::new(span1), sub_layout[2]);
//     frame.render_widget(Paragraph::new(span1), sub_layout[3]);
// }