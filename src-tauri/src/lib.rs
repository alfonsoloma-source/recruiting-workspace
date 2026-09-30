mod commands;
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
        .invoke_handler(tauri::generate_handler![
            commands::list_candidates,
            commands::create_candidate,
            commands::list_jobs,
            commands::create_job,
            commands::create_application,
            commands::update_application_stage,
            commands::add_candidate_note,
            commands::list_candidate_workspace,
            commands::list_job_workspace,
            commands::list_job_applications,
            commands::get_home_workspace,
            commands::list_interview_workspace,
            commands::create_interview,
            commands::create_action,
            commands::prepare_action,
            commands::request_action_confirmation,
            commands::confirm_action,
            commands::complete_action,
            commands::cancel_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running Recruiting Workspace");
}
