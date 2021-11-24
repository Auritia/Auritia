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
use std::sync::Arc;
use std::thread::spawn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetronomeEvent {
  Beat,
}

pub struct Engine {
  pub audio_manager: Arc<Mutex<AudioManager>>,
  pub clock: MetronomeHandle,
  pub metronome_sequence: SequenceInstanceHandle<MetronomeEvent>,
  pub loop_preview: bool,
}

impl Engine {
  pub fn new() -> Result<Engine, Box<dyn Error>> {
    let mut audio_manager = Arc::new(parking_lot::Mutex::new(
      AudioManager::new(AudioManagerSettings::default()).unwrap(),
    ));

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
      // Todo: make this be passed as a param from the constructor and localstorage
      loop_preview: false,
    });
  }

  pub fn preview_sample(&self, sample_path: String) -> Result<(), Box<dyn Error>> {
    let audio_manager = self.audio_manager.clone();

    spawn(move || {
      let mut sound_handle = audio_manager
        .lock()
        .load_sound(sample_path, SoundSettings::default())
        .unwrap();

      sound_handle.play(InstanceSettings::default()).unwrap();

      spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs_f64(sound_handle.duration()));
        audio_manager.lock().remove_sound(sound_handle.id());
      });
    });

    Ok(()) // and?
  }

  pub fn set_tempo(&mut self, tempo: f64) {
    self.clock.set_tempo(Tempo(tempo));
  }

  pub fn set_loop_preview(&mut self, state: bool) {
    self.loop_preview = state;
  }

  pub fn set_metronome(&mut self, state: bool) {
    if state {
      self.metronome_sequence.resume()
    } else {
      self.metronome_sequence.pause()
    };
  }
}
