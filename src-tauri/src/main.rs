#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use parking_lot::Mutex;
use std::str::FromStr;
use std::sync::RwLock;
use std::thread::spawn;
use std::time::SystemTime;
use tauri::Event;
use tauri::Manager;

mod engine;

use crate::engine::Engine;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cascade;

lazy_static! {
  static ref START_TIME: SystemTime = SystemTime::now();
  static ref IS_METRONOME_ENABLED: RwLock<bool> = RwLock::new(false);
  static ref IS_PLAYING: RwLock<bool> = RwLock::new(false);
  static ref SAMPLE_RATE: RwLock<u32> = RwLock::new(44100);
  static ref CHANNEL_COUNT: RwLock<u16> = RwLock::new(2);
}

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

fn main() {
  let mut engine = Mutex::new(Engine::new().unwrap());

  let app = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("Failed to build");

  cascade! {
    &app;
    ..listen_global("set_metronome", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      println!(
        "[EVENTS] got '{}' with payload {:?}",
        "set_metronome", value
      );
    });
    ..listen_global("tap_metronome", move |event| {
      println!("[EVENTS] got '{}'", "tap_metronome");
    });
    ..listen_global("set_bpm", move |event| {
      // This crashes when incementing by 0.10
      let value: i64 = FromStr::from_str(event.payload().unwrap()).unwrap();
      println!("[EVENTS] got '{}' with payload {:?}", "set_bpm", value);
    });
    ..listen_global("play", move |event| {
      engine.lock().metronome_handle.start();
      println!("[EVENTS] got '{}'", "play");
    });
    ..listen_global("stop", move |event| {
      println!("[EVENTS] got '{}'", "stop");
    });
  };

  app.run(|app_handle, e| ());
}
