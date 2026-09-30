use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub source: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub candidate_id: String,
    pub job_id: String,
    pub stage: String,
    pub status: String,
    pub applied_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interview {
    pub id: String,
    pub application_id: String,
    pub starts_at: String,
    pub ends_at: String,
    pub status: String,
    pub provider: Option<String>,
    pub external_event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub candidate_id: String,
    pub application_id: Option<String>,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub action_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub status: String,
    pub due_at: Option<String>,
    pub requires_confirmation: bool,
    pub payload: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}
