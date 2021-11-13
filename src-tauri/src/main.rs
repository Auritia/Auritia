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
  let empty_payload = Payload { message: "".into() };

  tauri::Builder::default()
    // Register the menu ribbon
    .menu(interface::create_menus())
    // Register Rust function to Vue
    .invoke_handler(tauri::generate_handler![engine::beep])
    // Pass the events to Vue
    .on_menu_event(move |event| {
      event
        .window()
        .emit(event.menu_item_id(), &empty_payload)
        .unwrap()
    })
    // Run the app
    .run(tauri::generate_context!())
    // Catch errors
    .expect("error while running tauri application");
}
