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
mod metronome;

use crate::engine::Engine;
#[macro_use]
extern crate cascade;
// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

fn main() {
  let builder = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("Failed to build");

  let app = builder.handle();
  let resource_path = app.path_resolver().resource_dir().unwrap();
  let (s, r) = unbounded::<String>();
  let engine = Arc::new(Mutex::new(Engine::new(s, resource_path).unwrap()));
  {
    let app = app.clone();

    spawn(move || {
      for value in r.iter() {
        app.emit_all("error", &value).expect("failed to emit error");
      }
    });
  }

  {
    let engine = engine.clone();

    app.listen_global("set_metronome", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      if let Err(_) = engine.lock().set_metronome(value) {
        engine.lock().tx.send("Couldn't set tempo".into());
      }
    });
  }

  {
    let engine = engine.clone();

    app.listen_global("set_loop_preview", move |event| {
      let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
      engine.lock().set_loop_preview(value);
    });
  }

  {
    let engine = engine.clone();

    // Replace this with a direct #[tauri::command] function
    app.listen_global("preview_sample", move |event| {
      let path = event.payload().unwrap();
      if let Err(error) = engine.lock().preview_sample(String::from(path)) {
        engine.lock().tx.send(error.to_string());
      }
    });
  }

  {
    app.listen_global("tap_metronome", move |_| {});
  }

  {
    let engine = engine.clone();

    app.listen_global("set_bpm", move |event| {
      // This crashes when incementing by 0.10
      let value: i64 = FromStr::from_str(event.payload().unwrap()).unwrap();
      if let Err(_) = engine.lock().set_tempo(value as f64) {
        engine.lock().tx.send("Couldn't set tempo".into());
      }
    });
  }
  {
    let engine = engine.clone();

    app.listen_global("play", move |_| {
      if let Err(_) = engine.lock().clock.start() {
        engine.lock().tx.send("Couldn't start playback".into());
      }
    });
  }

  {
    let engine = engine.clone();

    app.listen_global("stop", move |_| {
      if let Err(_) = engine.lock().clock.stop() {
        engine.lock().tx.send("Couldn't stop playback".into());
      }
    });
  }

  builder.run(|_, _| ());
}
