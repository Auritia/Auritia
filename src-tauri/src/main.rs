#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu;

fn main() {
  tauri::Builder::default()
    .on_page_load(|window, _payload| {
      let label = window.label().to_string();
      window.listen("clicked".to_string(), move |_payload| {
        println!("got 'clicked' event on window '{}'", label);
      });
    })
    .menu(menu::create_menus())
    .on_menu_event(|event| match event.menu_item_id() {
      "open" => {
        println!("open clicked!");
      }
      "save" => {
        println!("save Clicked!")
      }
      "save_as" => {
        println!("save_as Clicked!")
      }
      "render" => {
        println!("render Clicked!")
      }
      "project_info" => {
        println!("project_info Clicked!")
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
