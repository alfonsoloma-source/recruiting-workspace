use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;
use uuid::Uuid;

fn conn(app: &tauri::AppHandle) -> Result<Connection, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    crate::db::open(dir).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize)]
pub struct CandidateRow {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewCandidate {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub source: Option<String>,
}

#[tauri::command]
pub fn list_candidates(app: tauri::AppHandle) -> Result<Vec<CandidateRow>, String> {
    let db = conn(&app)?;
    let mut stmt = db.prepare("SELECT id,name,email,phone,source FROM candidates ORDER BY name").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(CandidateRow { id:r.get(0)?, name:r.get(1)?, email:r.get(2)?, phone:r.get(3)?, source:r.get(4)? })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_candidate(app: tauri::AppHandle, input: NewCandidate) -> Result<CandidateRow, String> {
    let db = conn(&app)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    db.execute("INSERT INTO candidates(id,name,email,phone,source,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?6,?6)",
        params![id,input.name,input.email,input.phone,input.source,now]).map_err(|e| e.to_string())?;
    Ok(CandidateRow { id, name:input.name, email:input.email, phone:input.phone, source:input.source })
}

#[derive(Debug, Serialize)]
pub struct JobRow { pub id:String, pub title:String, pub description:Option<String>, pub status:String }

#[derive(Debug, Deserialize)]
pub struct NewJob { pub title:String, pub description:Option<String> }

#[tauri::command]
pub fn list_jobs(app: tauri::AppHandle) -> Result<Vec<JobRow>, String> {
    let db=conn(&app)?;
    let mut stmt=db.prepare("SELECT id,title,description,status FROM jobs ORDER BY created_at DESC").map_err(|e|e.to_string())?;
    let rows=stmt.query_map([],|r|Ok(JobRow{id:r.get(0)?,title:r.get(1)?,description:r.get(2)?,status:r.get(3)?})).map_err(|e|e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())
}

#[tauri::command]
pub fn create_job(app: tauri::AppHandle, input: NewJob) -> Result<JobRow,String> {
    let db=conn(&app)?;
    let id=Uuid::new_v4().to_string();
    let now=Utc::now().to_rfc3339();
    db.execute("INSERT INTO jobs(id,title,description,status,created_at,updated_at) VALUES(?1,?2,?3,'open',?4,?4)",params![id,input.title,input.description,now]).map_err(|e|e.to_string())?;
    Ok(JobRow{id,title:input.title,description:input.description,status:"open".into()})
}

#[derive(Debug, Serialize)]
pub struct ApplicationRow { pub id:String, pub candidate_id:String, pub job_id:String, pub stage:String, pub status:String }

#[derive(Debug, Deserialize)]
pub struct NewApplication { pub candidate_id:String, pub job_id:String, pub stage:Option<String> }

#[tauri::command]
pub fn create_application(app: tauri::AppHandle,input:NewApplication)->Result<ApplicationRow,String>{
    let db=conn(&app)?;
    let id=Uuid::new_v4().to_string();
    let now=Utc::now().to_rfc3339();
    let stage=input.stage.unwrap_or_else(||"new".into());
    db.execute("INSERT INTO applications(id,candidate_id,job_id,stage,status,applied_at,updated_at) VALUES(?1,?2,?3,?4,'active',?5,?5)",params![id,input.candidate_id,input.job_id,stage,now]).map_err(|e|e.to_string())?;
    Ok(ApplicationRow{id,candidate_id:input.candidate_id,job_id:input.job_id,stage,status:"active".into()})
}

#[tauri::command]
pub fn update_application_stage(app:tauri::AppHandle,id:String,stage:String)->Result<(),String>{
    let db=conn(&app)?;
    let now=Utc::now().to_rfc3339();
    let changed=db.execute("UPDATE applications SET stage=?1,updated_at=?2 WHERE id=?3",params![stage,now,id]).map_err(|e|e.to_string())?;
    if changed==0 { return Err("Application not found".into()); }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct NewNote { pub candidate_id:String, pub application_id:Option<String>, pub content:String }

#[tauri::command]
pub fn add_candidate_note(app:tauri::AppHandle,input:NewNote)->Result<String,String>{
    let db=conn(&app)?;
    let id=Uuid::new_v4().to_string();
    let now=Utc::now().to_rfc3339();
    db.execute("INSERT INTO notes(id,candidate_id,application_id,content,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?5)",params![id,input.candidate_id,input.application_id,input.content,now]).map_err(|e|e.to_string())?;
    Ok(id)
}
