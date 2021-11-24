#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate ringbuf;

use std::str::FromStr;
use std::sync::RwLock;
use std::time::SystemTime;
use tauri::Manager;

static RING_BUFFER_SIZE: usize = 2048;

#[macro_use]
extern crate lazy_static;

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
  // Creates the webapp
  tauri::Builder::default()
    .setup(move |app| {
      app.listen_global("set_metronome", move |event| {
        let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
        println!(
          "[EVENTS] got '{}' with payload {:?}",
          "set_metronome", value
        );
      });

      app.listen_global("tap_metronome", move |event| {
        println!("[EVENTS] got '{}'", "tap_metronome");
      });

      app.listen_global("set_bpm", move |event| {
        // This crashes when incementing by 0.10
        let value: i64 = FromStr::from_str(event.payload().unwrap()).unwrap();
        println!("[EVENTS] got '{}' with payload {:?}", "set_bpm", value);
      });

      app.listen_global("play", move |event| {
        println!("[EVENTS] got '{}'", "play");
      });
      app.listen_global("stop", move |event| {
        println!("[EVENTS] got '{}'", "stop");
      });

      Ok(())
    })
    // Register Rust function to Vue
    // .invoke_handler(tauri::generate_handler![engine::create])
    // Run the app
    .run(tauri::generate_context!())
    // Catch errors
    .expect("error while running tauri application");
}
