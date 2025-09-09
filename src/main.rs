use std::error::Error;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "assets/audio/explode.wav");
    audio.add("lose", "assets/audio/lose.wav");
    audio.add("move", "assets/audio/move.wav");
    audio.add("pew", "assets/audio/pew.wav");
    audio.add("startup", "assets/audio/startup.wav");
    audio.add("win", "assets/audio/win.wav");
    audio.play("startup");

    //Cleanup
    audio.wait();
    Ok(())
}
