#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

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
    .on_page_load(|window, _payload| {
      let label = window.label().to_string();
      window.listen("clicked".to_string(), move |_payload| {
        println!("got 'clicked' event on window '{}'", label);
      });
    })
    .menu(interface::create_menus())
    .on_menu_event(move |event| match event.menu_item_id() {
      "open" => {
        event.window().emit("open", &empty_payload).unwrap();
        println!("open clicked!")
      }
      "save" => {
        event.window().emit("save", &empty_payload).unwrap();
        println!("save Clicked!")
      }
      "save_as" => {
        event.window().emit("save_as", &empty_payload).unwrap();
        println!("save_as Clicked!")
      }
      "render" => {
        event.window().emit("render", &empty_payload).unwrap();
        println!("render Clicked!")
      }
      "project_info" => {
        event.window().emit("project_info", &empty_payload).unwrap();
        println!("project_info Clicked!")
      }
      "preferences" => {
        event.window().emit("preferences", &empty_payload).unwrap();
        println!("preferences Clicked!")
      }
      "docs" => {
        event.window().emit("docs", &empty_payload).unwrap();
        println!("docs Clicked!")
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
