#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use parking_lot::Mutex;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
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
  let mut engine = Arc::new(Mutex::new(Engine::new().unwrap()));

  let engine1 = engine.clone();
  let engine2 = engine.clone();
  let engine3 = engine.clone();
  let engine4 = engine.clone();
  let engine5 = engine.clone();
  let engine6 = engine.clone();

  let app = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("Failed to build");

  cascade! {
    &app;
    ..listen_global("set_metronome", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      engine1.lock().set_metronome(value);
      println!(
        "[EVENTS] got '{}' with payload {:?}",
        "set_metronome", value
      );
    });
    ..listen_global("set_loop_preview", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      engine6.lock().set_loop_preview(value);
      println!(
        "[EVENTS] got '{}' with payload {:?}",
        "set_loop_preview", value
      );
    });
    // Replace this with a direct #[tauri::command] function
    ..listen_global("preview_sample", move |event| {
      let path = event.payload().unwrap();
      println!("[EVENTS] got '{}' path {}", "preview_sample", path);
      engine5.lock().preview_sample(String::from(path));
    });
    ..listen_global("tap_metronome", move |event| {
      println!("[EVENTS] got '{}'", "tap_metronome");
    });
    ..listen_global("set_bpm", move |event| {
      // This crashes when incementing by 0.10
      let value: i64 = FromStr::from_str(event.payload().unwrap()).unwrap();
      engine3.lock().set_tempo(value as f64);
      println!("[EVENTS] got '{}' with payload {:?}", "set_bpm", value);
    });
    ..listen_global("play", move |event| {
      engine2.lock().clock.start();
      println!("[EVENTS] got '{}'", "play");
    });
    ..listen_global("stop", move |event| {
      engine4.lock().clock.stop();
      println!("[EVENTS] got '{}'", "stop");
    });
  };

  app.run(|app_handle, e| ());
}
