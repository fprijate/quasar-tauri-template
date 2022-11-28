#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

#[tauri::command]
fn hello(name: &str) -> String {
  println!("Hello was called with an argument: {}", name);
  format!("Hello {} from Tauri!",name)
}

fn main() {
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![hello])
      .menu(
          tauri::Menu::os_default("Tauri Quasar Template").add_submenu(Submenu::new(
              "Help",
              Menu::with_items([CustomMenuItem::new(
                  "Tauri Home",
                  "Tauri Home",
              )
              .into()]),
          )),
      )
      .on_menu_event(|event| {
          let event_name = event.menu_item_id();
          match event_name {
              "Tauri Home" => {
                  let url = "https://tauri.app/".to_string();
                  shell::open(&event.window().shell_scope(), url, None).unwrap();
              }
              _ => {}
          }
      })
      .setup(|_app| {
          #[cfg(debug_assertions)]
          {
              let main_window = _app.get_window("main").unwrap();
              main_window.open_devtools();
          }
          Ok(())
      })
      .run(ctx)
      .expect("error while running tauri application");
}

