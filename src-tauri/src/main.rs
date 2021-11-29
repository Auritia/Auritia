#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crossbeam_channel::unbounded;
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
  // Clear the terminal
  print!("{}[2J", 27 as char);

  let panic_handler = arcmutex(PanicHandler::new());

  {
    let panic_handler = panic_handler.clone();
    std::panic::set_hook(Box::new(move |info| {
      if let Err(err) = panic_handler.lock().handle_panic(info) {
        eprintln!("failed to handle panic: {}", err);
      }
    }));
  }

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
  let fatal_log_path = app.path_resolver().resource_dir().unwrap().join("logs");

  panic_handler.lock().error_filepath = Some(fatal_log_path);

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
