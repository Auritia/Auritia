use std::error::Error;

use kira::instance::InstanceSettings;
use kira::manager::error::{LoadSoundError, StartSequenceError};
use kira::manager::AudioManager;
use kira::metronome::handle::MetronomeHandle;
use kira::sequence::handle::SequenceInstanceHandle;
use kira::sequence::{Sequence, SequenceInstanceSettings, SequenceSettings};
use kira::sound::handle::SoundHandle;
use kira::sound::SoundSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetronomeEvent {
  Beat,
}

pub struct Metronome {
  pub high_sound: SoundHandle,
  pub low_sound: SoundHandle,
  pub sequence: Sequence<MetronomeEvent>,
  pub sequence_handle: Option<SequenceInstanceHandle<MetronomeEvent>>,
}

impl Metronome {
  pub fn new(
    audio_manager: &mut AudioManager,
    high_path: impl AsRef<std::path::Path>,
    low_path: impl AsRef<std::path::Path>,
  ) -> Result<Metronome, LoadSoundError> {
    println!("{:?} {:?}", high_path.as_ref(), low_path.as_ref());
    let high_sound = audio_manager.load_sound(high_path, SoundSettings::default())?;
    let low_sound = audio_manager.load_sound(low_path, SoundSettings::default())?;

    let sequence = cascade! {
      Sequence::new(SequenceSettings::default());
      ..start_loop();
      ..emit(MetronomeEvent::Beat);
      ..play(&high_sound, InstanceSettings::default());
      ..wait(kira::Duration::Beats(1.0));
      ..emit(MetronomeEvent::Beat);
      ..play(&low_sound, InstanceSettings::default());
      ..wait(kira::Duration::Beats(1.0));
      ..emit(MetronomeEvent::Beat);
      ..play(&low_sound, InstanceSettings::default());
      ..wait(kira::Duration::Beats(1.0));
      ..emit(MetronomeEvent::Beat);
      ..play(&low_sound, InstanceSettings::default());
      ..wait(kira::Duration::Beats(1.0));
    };

    Ok(Metronome {
      high_sound,
      low_sound,
      sequence,
      sequence_handle: None,
    })
  }

  pub fn start(
    &mut self,
    audio_manager: &mut AudioManager,
    clock: &mut MetronomeHandle,
  ) -> Result<(), StartSequenceError> {
    self.sequence_handle = Some(audio_manager.start_sequence(
      self.sequence.clone(),
      SequenceInstanceSettings::new().metronome(clock.id()),
    )?);
    Ok(())
  }

  pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
    match self.sequence_handle {
      Some(ref mut handle) => {
        handle.stop()?;
        self.sequence_handle = None;
        Ok(())
      }
      None => Err("the metronome hasn't started you dumb cunt".into()),
    }
  }
}
