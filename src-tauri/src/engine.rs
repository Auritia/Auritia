use kira;
use kira::instance::InstanceSettings;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::SoundSettings;
use parking_lot::{self, Mutex};
use std::error::Error;

pub struct Engine {
  audio_manager: Mutex<AudioManager>,
}

impl Engine {
  pub fn new() -> Result<Engine, Box<dyn Error>> {
    let mut audio_manager =
      parking_lot::Mutex::new(AudioManager::new(AudioManagerSettings::default()).unwrap());

    let mut sound_handle = audio_manager
      .lock()
      .load_sound("sounds/metronome_low.wav", SoundSettings::default());

    sound_handle?.play(InstanceSettings::default());

    return Ok(Engine {
      audio_manager,
    });
  }
}
