#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crossbeam_channel::unbounded;
use parking_lot::Mutex;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::spawn;
use tauri::Manager;

mod engine;

use crate::engine::Engine;

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

fn main() {
  let engine = Arc::new(Mutex::new(Engine::new().unwrap()));

  let builder = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("Failed to build");

  let app = builder.handle();
  let (s, r) = unbounded::<String>();
  let engine_manager = (s, engine);

  {
    let app = app.clone();

    spawn(move || {
      for value in r.iter() {
        app.emit_all("error", &value).expect("failed to emit error");
      }
    });
  }

  {
    let (s, engine) = engine_manager.clone();

    app.listen_global("set_metronome", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      engine
        .lock()
        .set_metronome(value)
        .expect("Couldn't set tempo");
      println!(
        "[EVENTS] got '{}' with payload {:?}",
        "set_metronome", value
      );
    });
  }

  {
    let (s, engine) = engine_manager.clone();

    app.listen_global("set_loop_preview", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      engine.lock().set_loop_preview(value);
      println!(
        "[EVENTS] got '{}' with payload {:?}",
        "set_loop_preview", value
      );
    });
  }

  {
    let (s, engine) = engine_manager.clone();

    // Replace this with a direct #[tauri::command] function
    app.listen_global("preview_sample", move |event| {
      let path = event.payload().unwrap();
      println!("[EVENTS] got '{}' path {}", "preview_sample", path);
      if let Err(error) = engine.lock().preview_sample(String::from(path)) {
        s.send(error.to_string());
      }
    });
  }

  {
    app.listen_global("tap_metronome", move |_| {
      println!("[EVENTS] got '{}'", "tap_metronome");
    });
  }

  {
    let (s, engine) = engine_manager.clone();

    app.listen_global("set_bpm", move |event| {
      // This crashes when incementing by 0.10
      let value: i64 = FromStr::from_str(event.payload().unwrap()).unwrap();
      if let Err(_) = engine.lock().set_tempo(value as f64) {
        s.send("Couldn't set tempo".into());
      }
      println!("[EVENTS] got '{}' with payload {:?}", "set_bpm", value);
    });
  }
  {
    let (s, engine) = engine_manager.clone();

    app.listen_global("play", move |_| {
      if let Err(_) = engine.lock().clock.start() {
        s.send("Couldn't start playback".into());
      }
      println!("[EVENTS] got '{}'", "play");
    });
  }

  {
    let (s, engine) = engine_manager.clone();

    app.listen_global("stop", move |_| {
      if let Err(_) = engine.lock().clock.stop() {
        s.send("Couldn't stop playback".into());
      }
      println!("[EVENTS] got '{}'", "stop");
    });
  }

  builder.run(|_, _| ());
}
