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
  Beat,
}

pub struct Engine {
  pub audio_manager: Mutex<AudioManager>,
  pub clock: MetronomeHandle,
  pub metronome_sequence: SequenceInstanceHandle<MetronomeEvent>,
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

    let metronome_sequence = audio_manager.lock().start_sequence(
      {
        let mut sequence = Sequence::new(SequenceSettings::default());
        sequence.start_loop();
        sequence.emit(MetronomeEvent::Beat);
        sequence.play(&metronome_high_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Beat);
        sequence.play(&metronome_low_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Beat);
        sequence.play(&metronome_low_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence.emit(MetronomeEvent::Beat);
        sequence.play(&metronome_low_sound, InstanceSettings::default());
        sequence.wait(kira::Duration::Beats(1.0));
        sequence
      },
      SequenceInstanceSettings::new().metronome(&clock),
    )?;

    return Ok(Engine {
      audio_manager,
      clock,
      metronome_sequence,
    });
  }

  pub fn preview_sample(&mut self, sample_path: &str) {
    let mut sound_handle = self
      .audio_manager
      .lock()
      .load_sound(sample_path, SoundSettings::default())
      .expect("ass");

    sound_handle.play(InstanceSettings::default()).expect("ass");
  }

  pub fn set_tempo(&mut self, tempo: f64) {
    self.clock.set_tempo(Tempo(tempo));
  }

  pub fn set_metronome(&mut self, state: bool) {
    if state {
      self.metronome_sequence.resume()
    } else {
      self.metronome_sequence.pause()
    };
  }
}