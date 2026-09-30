mod db;
mod models;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            db::open(app_data_dir).map_err(|error| Box::<dyn std::error::Error>::from(error))?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Recruiting Workspace");
}
