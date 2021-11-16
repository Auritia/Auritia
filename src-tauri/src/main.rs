#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod engine;
mod interface;

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  engine::create();

  tauri::Builder::default()
    // Register Rust function to Vue
    // .invoke_handler(tauri::generate_handler![engine::beep])
    // Run the app
    .run(tauri::generate_context!())
    // Catch errors
    .expect("error while running tauri application");
}
