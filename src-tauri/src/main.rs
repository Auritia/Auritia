#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crossbeam_channel::unbounded;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::spawn;
use tauri::Manager;

mod engine;
mod metronome;
mod panic_handler;
mod util;

use crate::engine::*;
use crate::panic_handler::PanicHandler;
use crate::util::*;
#[macro_use]
extern crate cascade;

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

fn main() {
  let error_filepath_cell = Arc::new(OnceCell::new());
  let panic_handler = PanicHandler::new(error_filepath_cell.clone());

  std::panic::set_hook(Box::new(move |info| {
    if let Err(err) = panic_handler.handle_panic(info) {
      eprintln!("failed to handle panic: {}", err);
    }
  }));

  let builder = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      set_metronome,
      set_bpm,
      set_loop_preview,
      preview_sample,
      play,
      stop,
    ])
    .build(tauri::generate_context!())
    .expect("Failed to build");

  let app = builder.handle();
  let resource_path = app.path_resolver().resource_dir().unwrap();
  let fatal_log_path = resource_path.join("logs");

  error_filepath_cell.set(fatal_log_path).unwrap();

  let (s, r) = unbounded::<String>();
  let engine = arcmutex(Engine::new(s, resource_path).unwrap());

  {
    let engine = engine.clone();
    builder.manage(engine);
  }

  {
    let app = app.clone();
    spawn(move || {
      for value in r.iter() {
        app.emit_all("error", &value).expect("failed to emit error");
      }
    });
  }

  builder.run(|_, _| ());
}
