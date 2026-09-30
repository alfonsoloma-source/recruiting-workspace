use keyring::Entry;
use serde::Serialize;

const SERVICE: &str = "recruiting-workspace";

fn supported(provider: &str) -> bool {
    matches!(provider, "openai" | "anthropic" | "gemini")
}

fn entry(provider: &str) -> Result<Entry, String> {
    if !supported(provider) { return Err("Unsupported credential provider".into()); }
    Entry::new(SERVICE, provider).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize)]
pub struct CredentialStatus {
    pub provider: String,
    pub configured: bool,
}

#[tauri::command]
pub fn credential_status(provider: String) -> Result<CredentialStatus, String> {
    let configured = entry(&provider)?.get_password().map(|v| !v.trim().is_empty()).unwrap_or(false);
    Ok(CredentialStatus { provider, configured })
}

#[tauri::command]
pub fn save_provider_credential(provider: String, secret: String) -> Result<CredentialStatus, String> {
    if secret.trim().is_empty() { return Err("Credential cannot be empty".into()); }
    entry(&provider)?.set_password(secret.trim()).map_err(|e| e.to_string())?;
    Ok(CredentialStatus { provider, configured: true })
}

#[tauri::command]
pub fn delete_provider_credential(provider: String) -> Result<CredentialStatus, String> {
    let e = entry(&provider)?;
    match e.delete_credential() {
        Ok(_) | Err(keyring::Error::NoEntry) => Ok(CredentialStatus { provider, configured: false }),
        Err(error) => Err(error.to_string()),
    }
}

pub fn read_provider_credential(provider: &str) -> Result<String, String> {
    entry(provider)?.get_password().map_err(|e| e.to_string())
}
