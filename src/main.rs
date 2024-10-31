use std::{error::Error, io};

use clap::Parser;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

/// A Rust interface to summarise LOFAR H5parm calibration tables.
#[derive(Parser, Debug)]
#[command(name = "LOFAR-H5stat")]
#[command(author = "Frits Sweijen")]
#[command(version = "0.0.0")]
#[command(
    help_template = "{name} \nVersion: {version} \nAuthor: {author}\n{about-section} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
// #[clap(author="Author Name", version, about="")]
struct Args {
    /// H5parm to summarise.
    ms: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout(); // This is a special case. Normally using stdout is fine
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(args.ms);
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            //app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('?') => {
                        app.current_screen = CurrentScreen::Help;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Tab => {
                        app.toggle_editing(true);
                    }
                    KeyCode::BackTab => {
                        app.toggle_editing(false);
                    }
                    KeyCode::Up => {
                        app.decrease_soltab(1, "data");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::Down => {
                        app.increase_soltab(1, "data");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::Char('k') => {
                        app.decrease_soltab(1, "data");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::Char('K') => {
                        app.decrease_soltab(10, "data");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::Char('j') => {
                        app.increase_soltab(1, "data");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::PageUp => {
                        app.decrease_soltab(1, "view");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::PageDown => {
                        app.increase_soltab(1, "view");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::Char('J') => {
                        app.increase_soltab(10, "data");
                        app.update_soltabs();
                        match app.currently_editing {
                            CurrentlyEditing::Table => app.select(true),
                            CurrentlyEditing::Information => app.select(false),
                            _ => {},
                        }
                    }
                    KeyCode::Enter => {
                        app.select(true);
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        //return Ok(false);
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::Help => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
            }
        }
    }
}
