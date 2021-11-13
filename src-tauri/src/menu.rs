use tauri::{CustomMenuItem, Menu, Submenu};

pub fn create_menus() -> tauri::Menu {
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
  let settings_menu = Submenu::new(
    "Settings",
    Menu::new().add_item(CustomMenuItem::new(
      "preferences".to_string(),
      "Preferences...",
    )),
  );
  let help_menu = Submenu::new(
    "Help",
    Menu::new().add_item(CustomMenuItem::new("docs".to_string(), "Documentation...")),
  );
  // configure the menu
  let menu = Menu::new()
    .add_submenu(file_menu)
    .add_submenu(settings_menu)
    .add_submenu(help_menu);

  return menu;
}
