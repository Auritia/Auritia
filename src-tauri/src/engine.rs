use kira;
use kira::instance::InstanceSettings;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::metronome::handle::MetronomeHandle;
use kira::metronome::MetronomeSettings;
use kira::sequence::handle::SequenceInstanceHandle;
use kira::sequence::{Sequence, SequenceInstanceSettings, SequenceSettings};
use kira::sound::handle::SoundHandle;
use kira::sound::SoundSettings;
use kira::Tempo;
use parking_lot::{self, Mutex};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetronomeEvent {
  Metronome,
}

pub struct Engine {
  audio_manager: Mutex<AudioManager>,
  pub metronome_handle: MetronomeHandle,
}

impl Engine {
  pub fn new() -> Result<Engine, Box<dyn Error>> {
    let mut audio_manager =
      parking_lot::Mutex::new(AudioManager::new(AudioManagerSettings::default()).unwrap());

    let metronome_sound_handle = audio_manager
      .lock()
      .load_sound("sounds/metronome_low.wav", SoundSettings::default())?;

    let mut metronome_handle = audio_manager
      .lock()
      .add_metronome(MetronomeSettings::new().tempo(Tempo(150.0)))?;

    let sequence_handle = audio_manager.lock().start_sequence(
      {
        let mut sequence = Sequence::new(SequenceSettings::default());
        sequence.start_loop();
        sequence.play(&metronome_sound_handle, InstanceSettings::default());
        sequence.emit(MetronomeEvent::Metronome);
        sequence.wait(kira::Duration::Beats(1.0));
        sequence
      },
      SequenceInstanceSettings::new().metronome(&metronome_handle),
    )?;

    return Ok(Engine {
      audio_manager,
      metronome_handle,
    });
  }
}
