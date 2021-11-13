#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu};

fn main() {
  let file_menu = Submenu::new(
    "File",
    Menu::new()
      .add_item(CustomMenuItem::new("open".to_string(), "Open"))
      .add_item(CustomMenuItem::new("save".to_string(), "Save Project"))
      .add_item(CustomMenuItem::new(
        "save_as".to_string(),
        "Save Project As...",
      ))
      .add_item(CustomMenuItem::new("render".to_string(), "Render Audio"))
      .add_item(CustomMenuItem::new(
        "project_info".to_string(),
        "Project Info",
      )),
  );
  // configure the menu
  let menu = Menu::new().add_submenu(file_menu);

  tauri::Builder::default()
    .on_page_load(|window, _payload| {
      let label = window.label().to_string();
      window.listen("clicked".to_string(), move |_payload| {
        println!("got 'clicked' event on window '{}'", label);
      });
    })
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
