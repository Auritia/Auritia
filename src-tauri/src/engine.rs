use kira;
use kira::instance::InstanceSettings;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::metronome::handle::MetronomeHandle;
use kira::metronome::MetronomeSettings;
use kira::sequence::handle::SequenceInstanceHandle;
use kira::sequence::{Sequence, SequenceInstanceSettings, SequenceSettings};
use kira::sound::SoundSettings;
use kira::Tempo;
use parking_lot::{self, Mutex};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetronomeEvent {
  Metronome,
}

pub struct Engine {
  pub audio_manager: Mutex<AudioManager>,
  pub clock: MetronomeHandle,
  pub sequence_handle: SequenceInstanceHandle<MetronomeEvent>,
}

impl Engine {
  pub fn new() -> Result<Engine, Box<dyn Error>> {
    let mut audio_manager =
      parking_lot::Mutex::new(AudioManager::new(AudioManagerSettings::default()).unwrap());

    let metronome_high_sound = audio_manager
      .lock()
      .load_sound("sounds/metronome_high.wav", SoundSettings::default())?;

    let metronome_low_sound = audio_manager
      .lock()
      .load_sound("sounds/metronome_low.wav", SoundSettings::default())?;

    let mut clock = audio_manager
      .lock()
      .add_metronome(MetronomeSettings::new().tempo(Tempo(120.0)))?;

    let sequence_handle = audio_manager.lock().start_sequence(
      {
        let mut sequence = Sequence::new(SequenceSettings::default());
        sequence.start_loop();
        sequence.play(&metronome_high_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Metronome);
        sequence.play(&metronome_low_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Metronome);
        sequence.play(&metronome_low_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Metronome);
        sequence.play(&metronome_low_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Metronome);
        sequence
      },
      SequenceInstanceSettings::new().metronome(&clock),
    )?;

    return Ok(Engine {
      audio_manager,
      clock,
      sequence_handle,
    });
  }

  pub fn set_tempo(&mut self, tempo: f64) {
    self.clock.set_tempo(Tempo(tempo));
  }

  pub fn set_metronome(&mut self, state: bool) {
    if state {
      self.sequence_handle.resume()
    } else {
      self.sequence_handle.pause()
    };
  }
}
