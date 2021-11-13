#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod interface;

use tauri::{Manager, Window};

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // listen to the `event-name` (emitted on any window)
      let id = app.listen_global("event-name", |event| {
        println!("got event-name with payload {:?}", event.payload());
      });
      // unlisten to the event using the `id` returned on the `listen_global` function
      // an `once_global` API is also exposed on the `App` struct
      app.unlisten(id);

      // emit the `event-name` event to all webview windows on the frontend
      app
        .emit_all(
          "event-name",
          Payload {
            message: "Tauri is awesome!".into(),
          },
        )
        .unwrap();
      Ok(())
    })
    .on_page_load(|window, _payload| {
      let label = window.label().to_string();
      window.listen("clicked".to_string(), move |_payload| {
        println!("got 'clicked' event on window '{}'", label);
      });
    })
    .menu(interface::create_menus())
    .on_menu_event(|event| match event.menu_item_id() {
      "open" => {
        event
          .window()
          .emit("open", Payload { message: "".into() })
          .unwrap();
        println!("open clicked!")
      }
      "save" => {
        event
          .window()
          .emit("save", Payload { message: "".into() })
          .unwrap();
        println!("save Clicked!")
      }
      "save_as" => {
        event
          .window()
          .emit("save_as", Payload { message: "".into() })
          .unwrap();
        println!("save_as Clicked!")
      }
      "render" => {
        event
          .window()
          .emit("render", Payload { message: "".into() })
          .unwrap();
        println!("render Clicked!")
      }
      "project_info" => {
        event
          .window()
          .emit("project_info", Payload { message: "".into() })
          .unwrap();
        println!("project_info Clicked!")
      }
      "preferences" => {
        event
          .window()
          .emit("preferences", Payload { message: "".into() })
          .unwrap();
        println!("preferences Clicked!")
      }
      "docs" => {
        event
          .window()
          .emit("docs", Payload { message: "".into() })
          .unwrap();
        println!("docs Clicked!")
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
