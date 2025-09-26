use std::{error::Error, time::Duration};

use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use rusty_audio::Audio;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "assets/audio/explode.wav");
    audio.add("lose", "assets/audio/lose.wav");
    audio.add("move", "assets/audio/move.wav");
    audio.add("pew", "assets/audio/pew.wav");
    audio.add("startup", "assets/audio/startup.wav");
    audio.add("win", "assets/audio/win.wav");
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
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }
    

    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode();
    Ok(())
}
