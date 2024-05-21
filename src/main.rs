use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tokio::runtime::Runtime;

mod fuzzer;
use fuzzer::fuzz_http_request;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(size);

            let block = Block::default().title("AIF: AI Fuzzer").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let instructions = Paragraph::new("Press f to start fuzzing")
                .style(Style::default().fg(Color::White));
            f.render_widget(instructions, chunks[1])
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('f') => {
                    // Trigger fuzzing
                    runtime.block_on(fuzz_http_request("raw http req", "jovia endpoint"))?;
                },
                _ => {}
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
