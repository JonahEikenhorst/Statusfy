
use tauri::Window;

#[tauri::command]
fn close_splashscreen(window: Window) {
    // Close the "splashscreen" window
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show the "statusfy" window
    if let Some(statusfy) = window.get_window("statusfy") {
        statusfy.show().unwrap();
    }
}

use tauri::Manager;
fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let splashscreen_window = app.get_window("splashscreen").unwrap();
      let statusfy = app.get_window("statusfy").unwrap();
      // we perform the initialization code on a new task so the app doesn't freeze
      tauri::async_runtime::spawn(async move {
        // temp sleep to simulate a long initialization process
        println!("Initializing...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Done initializing.");

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        statusfy.show().unwrap();
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

