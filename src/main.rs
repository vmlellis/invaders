use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::{ExecutableCommand, event, terminal};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let dir_path = "audio/contributions/gekh/";
    let mut audio = Audio::new();
    audio.add("explode", &format!("{}explode.wav", dir_path));
    audio.add("lose", &format!("{}lose.wav", dir_path));
    audio.add("move", &format!("{}move.wav", dir_path));
    audio.add("pew", &format!("{}pew.wav", dir_path));
    audio.add("startup", &format!("{}startup.wav", dir_path));
    audio.add("win", &format!("{}win.wav", dir_path));
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game Loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
