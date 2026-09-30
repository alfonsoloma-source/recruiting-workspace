use serde::{Deserialize, Serialize};
use tauri_plugin_store::StoreExt;

const SETTINGS_FILE: &str = "settings.json";
const AI_PROVIDER_KEY: &str = "ai_provider";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub provider: String,
    pub model: Option<String>,
}

impl Default for AiSettings {
    fn default() -> Self {
        Self { provider: "local-template".into(), model: None }
    }
}

#[tauri::command]
pub fn get_ai_settings(app: tauri::AppHandle) -> Result<AiSettings, String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let Some(value) = store.get(AI_PROVIDER_KEY) else { return Ok(AiSettings::default()); };
    serde_json::from_value(value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_ai_settings(app: tauri::AppHandle, settings: AiSettings) -> Result<AiSettings, String> {
    match settings.provider.as_str() {
        "local-template" | "openai" | "anthropic" | "gemini" => {}
        _ => return Err("Unsupported AI provider".into()),
    }
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    store.set(AI_PROVIDER_KEY, serde_json::to_value(&settings).map_err(|e| e.to_string())?);
    store.save().map_err(|e| e.to_string())?;
    Ok(settings)
}
