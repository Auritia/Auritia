use crate::metronome;
use crossbeam_channel::Sender;
use kira::instance::InstanceSettings;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::metronome::handle::MetronomeHandle;
use kira::metronome::MetronomeSettings;
use kira::sound::SoundSettings;
use kira::Tempo;
use kira::{self, CommandError};
use parking_lot::{self, Mutex};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::spawn;

fn sleep(secs: f64) {
  return std::thread::sleep(std::time::Duration::from_secs_f64(secs));
}

pub struct Engine {
  pub resource_root: PathBuf,
  pub tx: Sender<String>,
  pub audio_manager: Arc<Mutex<AudioManager>>,
  pub clock: MetronomeHandle,
  pub metronome: metronome::Metronome,
  pub loop_preview: bool,
}

impl Engine {
  pub fn new(tx: Sender<String>, resource_root: PathBuf) -> Result<Engine, Box<dyn Error>> {
    let resource_root = resource_root;
    let tx = tx.clone();
    let mut audio_manager = Arc::new(parking_lot::Mutex::new(
      AudioManager::new(AudioManagerSettings::default()).unwrap(),
    ));

    let mut clock = audio_manager
      .lock()
      .add_metronome(MetronomeSettings::new().tempo(Tempo(120.0)))?;

    let metronome = metronome::Metronome::new(
      &mut audio_manager.lock(),
      resource_root.join("sounds/metronome_high.wav"),
      resource_root.join("sounds/metronome_low.wav"),
    )?;

    return Ok(Engine {
      resource_root,
      audio_manager,
      clock,
      metronome,
      tx,
      // Todo: make this be passed as a param from the constructor and localstorage
      loop_preview: false,
    });
  }

  pub fn preview_sample(&self, sample_path: String) -> Result<(), Box<dyn Error>> {
    let audio_manager = self.audio_manager.clone();

    let tx = self.tx.clone();
    spawn(move || {
      let load_result = audio_manager
        .lock()
        .load_sound(sample_path, SoundSettings::default());

      match load_result {
        Ok(mut sound_handle) => {
          let play_result = sound_handle.play(InstanceSettings::default());
          if let Err(e) = play_result {
            tx.send(e.to_string()).unwrap();
          };
          sleep(sound_handle.duration());
          audio_manager
            .lock()
            .remove_sound(sound_handle.id())
            .unwrap();
        }
        Err(e) => tx.send(e.to_string()).unwrap(),
      }
    });

    Ok(()) // and?
  }

  pub fn set_tempo(&mut self, tempo: f64) -> Result<(), CommandError> {
    return self.clock.set_tempo(Tempo(tempo));
  }

  pub fn set_loop_preview(&mut self, state: bool) {
    self.loop_preview = state;
  }

  pub fn set_metronome(&mut self, state: bool) -> Result<(), Box<dyn Error>> {
    if state {
      let mut audio_manager = self.audio_manager.lock();
      self.metronome.start(&mut audio_manager, &mut self.clock)?;
    } else {
      self.metronome.stop()?;
    };
    Ok(())
  }
}
