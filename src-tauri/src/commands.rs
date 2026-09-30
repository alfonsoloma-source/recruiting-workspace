use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
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


#[derive(Debug, Serialize)]
pub struct CandidateWorkspaceRow {
    pub application_id: String,
    pub candidate_id: String,
    pub candidate_name: String,
    pub email: Option<String>,
    pub job_id: String,
    pub job_title: String,
    pub stage: String,
    pub status: String,
    pub updated_at: String,
}

#[tauri::command]
pub fn list_candidate_workspace(app: tauri::AppHandle) -> Result<Vec<CandidateWorkspaceRow>, String> {
    let db = conn(&app)?;
    let mut stmt = db.prepare(
        "SELECT a.id,c.id,c.name,c.email,j.id,j.title,a.stage,a.status,a.updated_at
         FROM applications a
         JOIN candidates c ON c.id=a.candidate_id
         JOIN jobs j ON j.id=a.job_id
         ORDER BY a.updated_at DESC"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(CandidateWorkspaceRow {
        application_id:r.get(0)?, candidate_id:r.get(1)?, candidate_name:r.get(2)?,
        email:r.get(3)?, job_id:r.get(4)?, job_title:r.get(5)?, stage:r.get(6)?,
        status:r.get(7)?, updated_at:r.get(8)?
    })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}


#[derive(Debug, Serialize)]
pub struct JobWorkspaceRow {
    pub job_id: String,
    pub title: String,
    pub status: String,
    pub total: i64,
    pub new_count: i64,
    pub screening_count: i64,
    pub interview_count: i64,
    pub finalist_count: i64,
}

#[tauri::command]
pub fn list_job_workspace(app: tauri::AppHandle) -> Result<Vec<JobWorkspaceRow>, String> {
    let db = conn(&app)?;
    let mut stmt = db.prepare(
        "SELECT j.id,j.title,j.status,
         COUNT(a.id),
         SUM(CASE WHEN LOWER(a.stage) IN ('new','nuevo') THEN 1 ELSE 0 END),
         SUM(CASE WHEN LOWER(a.stage)='screening' THEN 1 ELSE 0 END),
         SUM(CASE WHEN LOWER(a.stage) IN ('interview','entrevista','técnica','tecnica') THEN 1 ELSE 0 END),
         SUM(CASE WHEN LOWER(a.stage) IN ('finalist','finalista') THEN 1 ELSE 0 END)
         FROM jobs j
         LEFT JOIN applications a ON a.job_id=j.id AND a.status='active'
         GROUP BY j.id,j.title,j.status
         ORDER BY j.created_at DESC"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(JobWorkspaceRow {
        job_id:r.get(0)?, title:r.get(1)?, status:r.get(2)?, total:r.get(3)?,
        new_count:r.get::<_,Option<i64>>(4)?.unwrap_or(0),
        screening_count:r.get::<_,Option<i64>>(5)?.unwrap_or(0),
        interview_count:r.get::<_,Option<i64>>(6)?.unwrap_or(0),
        finalist_count:r.get::<_,Option<i64>>(7)?.unwrap_or(0)
    })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_job_applications(app: tauri::AppHandle, job_id: String) -> Result<Vec<CandidateWorkspaceRow>, String> {
    let db = conn(&app)?;
    let mut stmt = db.prepare(
        "SELECT a.id,c.id,c.name,c.email,j.id,j.title,a.stage,a.status,a.updated_at
         FROM applications a
         JOIN candidates c ON c.id=a.candidate_id
         JOIN jobs j ON j.id=a.job_id
         WHERE j.id=?1
         ORDER BY a.updated_at DESC"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([job_id], |r| Ok(CandidateWorkspaceRow {
        application_id:r.get(0)?, candidate_id:r.get(1)?, candidate_name:r.get(2)?,
        email:r.get(3)?, job_id:r.get(4)?, job_title:r.get(5)?, stage:r.get(6)?,
        status:r.get(7)?, updated_at:r.get(8)?
    })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}


#[derive(Debug, Serialize)]
pub struct AttentionRow {
    pub action_id: String,
    pub application_id: String,
    pub candidate_id: String,
    pub candidate_name: String,
    pub job_title: String,
    pub stage: String,
    pub action_type: String,
    pub due_at: Option<String>,
    pub requires_confirmation: bool,
    pub status: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct HomeWorkspace {
    pub pending_count: i64,
    pub interview_today_count: i64,
    pub new_count: i64,
    pub attention: Vec<AttentionRow>,
    pub new_candidates: Vec<CandidateWorkspaceRow>,
}

#[tauri::command]
pub fn get_home_workspace(app: tauri::AppHandle) -> Result<HomeWorkspace, String> {
    let db = conn(&app)?;

    let pending_count: i64 = db.query_row(
        "SELECT COUNT(*) FROM actions WHERE status IN ('requested','prepared','awaiting_confirmation')",
        [], |r| r.get(0)
    ).map_err(|e| e.to_string())?;

    let interview_today_count: i64 = db.query_row(
        "SELECT COUNT(*) FROM interviews WHERE status='scheduled' AND date(starts_at)=date('now','localtime')",
        [], |r| r.get(0)
    ).map_err(|e| e.to_string())?;

    let new_count: i64 = db.query_row(
        "SELECT COUNT(*) FROM applications WHERE status='active' AND LOWER(stage) IN ('new','nuevo')",
        [], |r| r.get(0)
    ).map_err(|e| e.to_string())?;

    let mut attention_stmt = db.prepare(
        "SELECT ac.id,a.id,c.id,c.name,j.title,a.stage,ac.type,ac.due_at,ac.requires_confirmation,ac.status,ac.payload
         FROM actions ac
         JOIN applications a ON ac.entity_type='application' AND ac.entity_id=a.id
         JOIN candidates c ON c.id=a.candidate_id
         JOIN jobs j ON j.id=a.job_id
         WHERE ac.status IN ('requested','prepared','awaiting_confirmation')
         ORDER BY CASE WHEN ac.due_at IS NULL THEN 1 ELSE 0 END, ac.due_at ASC"
    ).map_err(|e| e.to_string())?;
    let attention = attention_stmt.query_map([], |r| Ok(AttentionRow {
        action_id:r.get(0)?, application_id:r.get(1)?, candidate_id:r.get(2)?,
        candidate_name:r.get(3)?, job_title:r.get(4)?, stage:r.get(5)?,
        action_type:r.get(6)?, due_at:r.get(7)?, requires_confirmation:r.get::<_,i64>(8)? != 0,
        status:r.get(9)?, payload:serde_json::from_str::<serde_json::Value>(&r.get::<_,String>(10)?).unwrap_or_else(|_| serde_json::json!({}))
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;

    let mut new_stmt = db.prepare(
        "SELECT a.id,c.id,c.name,c.email,j.id,j.title,a.stage,a.status,a.updated_at
         FROM applications a JOIN candidates c ON c.id=a.candidate_id JOIN jobs j ON j.id=a.job_id
         WHERE a.status='active' AND LOWER(a.stage) IN ('new','nuevo')
         ORDER BY a.updated_at DESC LIMIT 5"
    ).map_err(|e| e.to_string())?;
    let new_candidates = new_stmt.query_map([], |r| Ok(CandidateWorkspaceRow {
        application_id:r.get(0)?, candidate_id:r.get(1)?, candidate_name:r.get(2)?,
        email:r.get(3)?, job_id:r.get(4)?, job_title:r.get(5)?, stage:r.get(6)?,
        status:r.get(7)?, updated_at:r.get(8)?
    })).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())?;

    Ok(HomeWorkspace { pending_count, interview_today_count, new_count, attention, new_candidates })
}


#[derive(Debug, Serialize)]
pub struct InterviewWorkspaceRow {
    pub interview_id: String,
    pub application_id: String,
    pub candidate_id: String,
    pub candidate_name: String,
    pub job_title: String,
    pub stage: String,
    pub starts_at: String,
    pub ends_at: String,
    pub status: String,
    pub provider: Option<String>,
}

#[tauri::command]
pub fn list_interview_workspace(app: tauri::AppHandle) -> Result<Vec<InterviewWorkspaceRow>, String> {
    let db = conn(&app)?;
    let mut stmt = db.prepare(
        "SELECT i.id,a.id,c.id,c.name,j.title,a.stage,i.starts_at,i.ends_at,i.status,i.provider
         FROM interviews i
         JOIN applications a ON a.id=i.application_id
         JOIN candidates c ON c.id=a.candidate_id
         JOIN jobs j ON j.id=a.job_id
         WHERE i.status IN ('scheduled','needs_reschedule')
         ORDER BY i.starts_at ASC"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(InterviewWorkspaceRow {
        interview_id:r.get(0)?, application_id:r.get(1)?, candidate_id:r.get(2)?,
        candidate_name:r.get(3)?, job_title:r.get(4)?, stage:r.get(5)?,
        starts_at:r.get(6)?, ends_at:r.get(7)?, status:r.get(8)?, provider:r.get(9)?
    })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>,_>>().map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct NewInterview {
    pub application_id: String,
    pub starts_at: String,
    pub ends_at: String,
    pub provider: Option<String>,
}

#[tauri::command]
pub fn create_interview(app: tauri::AppHandle, input: NewInterview) -> Result<String, String> {
    let db = conn(&app)?;
    let id = Uuid::new_v4().to_string();
    db.execute(
        "INSERT INTO interviews(id,application_id,starts_at,ends_at,status,provider) VALUES(?1,?2,?3,?4,'scheduled',?5)",
        params![id,input.application_id,input.starts_at,input.ends_at,input.provider]
    ).map_err(|e| e.to_string())?;
    Ok(id)
}


#[derive(Debug, Serialize)]
pub struct ActionRow {
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

fn get_action(db: &Connection, id: &str) -> Result<ActionRow, String> {
    db.query_row(
        "SELECT id,type,entity_type,entity_id,status,due_at,requires_confirmation,payload,created_at,updated_at FROM actions WHERE id=?1",
        [id],
        |r| {
            let raw: String = r.get(7)?;
            Ok(ActionRow {
                id:r.get(0)?, action_type:r.get(1)?, entity_type:r.get(2)?, entity_id:r.get(3)?,
                status:r.get(4)?, due_at:r.get(5)?, requires_confirmation:r.get::<_,i64>(6)? != 0,
                payload:serde_json::from_str(&raw).unwrap_or(serde_json::json!({})),
                created_at:r.get(8)?, updated_at:r.get(9)?
            })
        }
    ).map_err(|e| e.to_string())
}

fn allowed_transition(from: &str, to: &str) -> bool {
    matches!((from,to),
        ("requested","prepared") |
        ("prepared","awaiting_confirmation") |
        ("awaiting_confirmation","executing") |
        ("executing","completed") |
        ("requested","cancelled") |
        ("prepared","cancelled") |
        ("awaiting_confirmation","cancelled") |
        ("executing","failed")
    )
}

#[derive(Debug, Deserialize)]
pub struct NewAction {
    pub action_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub due_at: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub payload: Option<serde_json::Value>,
}

#[tauri::command]
pub fn create_action(app: tauri::AppHandle, input: NewAction) -> Result<ActionRow, String> {
    let db=conn(&app)?;
    let id=Uuid::new_v4().to_string();
    let now=Utc::now().to_rfc3339();
    let requires=input.requires_confirmation.unwrap_or(false);
    let payload=input.payload.unwrap_or(serde_json::json!({})).to_string();
    db.execute(
        "INSERT INTO actions(id,type,entity_type,entity_id,status,due_at,requires_confirmation,payload,created_at,updated_at)
         VALUES(?1,?2,?3,?4,'requested',?5,?6,?7,?8,?8)",
        params![id,input.action_type,input.entity_type,input.entity_id,input.due_at,requires as i64,payload,now]
    ).map_err(|e|e.to_string())?;
    get_action(&db,&id)
}

#[tauri::command]
pub fn prepare_action(app: tauri::AppHandle, id:String, payload:serde_json::Value) -> Result<ActionRow,String> {
    let db=conn(&app)?;
    let current=get_action(&db,&id)?;
    if current.status!="requested" && current.status!="prepared" { return Err(format!("Invalid action transition: {} -> prepared",current.status)); }
    let now=Utc::now().to_rfc3339();
    db.execute("UPDATE actions SET status='prepared',payload=?1,updated_at=?2 WHERE id=?3",params![payload.to_string(),now,id]).map_err(|e|e.to_string())?;
    get_action(&db,&id)
}

#[tauri::command]
pub fn request_action_confirmation(app:tauri::AppHandle,id:String)->Result<ActionRow,String>{
    let db=conn(&app)?;
    let current=get_action(&db,&id)?;
    if !current.requires_confirmation { return Err("This action does not require confirmation".into()); }
    if !allowed_transition(&current.status,"awaiting_confirmation") { return Err(format!("Invalid action transition: {} -> awaiting_confirmation",current.status)); }
    let now=Utc::now().to_rfc3339();
    db.execute("UPDATE actions SET status='awaiting_confirmation',updated_at=?1 WHERE id=?2",params![now,id]).map_err(|e|e.to_string())?;
    get_action(&db,&id)
}

#[tauri::command]
pub fn confirm_action(app:tauri::AppHandle,id:String)->Result<ActionRow,String>{
    let db=conn(&app)?;
    let current=get_action(&db,&id)?;
    if !current.requires_confirmation { return Err("Confirmation is not required for this action".into()); }
    if !allowed_transition(&current.status,"executing") { return Err(format!("Invalid action transition: {} -> executing",current.status)); }
    let now=Utc::now().to_rfc3339();
    db.execute("UPDATE actions SET status='executing',updated_at=?1 WHERE id=?2",params![now,id]).map_err(|e|e.to_string())?;
    get_action(&db,&id)
}

#[tauri::command]
pub fn complete_action(app:tauri::AppHandle,id:String)->Result<ActionRow,String>{
    let db=conn(&app)?;
    let current=get_action(&db,&id)?;
    if !allowed_transition(&current.status,"completed") { return Err(format!("Invalid action transition: {} -> completed",current.status)); }
    let now=Utc::now().to_rfc3339();
    db.execute("UPDATE actions SET status='completed',updated_at=?1 WHERE id=?2",params![now,id]).map_err(|e|e.to_string())?;
    get_action(&db,&id)
}

#[tauri::command]
pub fn cancel_action(app:tauri::AppHandle,id:String)->Result<ActionRow,String>{
    let db=conn(&app)?;
    let current=get_action(&db,&id)?;
    if !allowed_transition(&current.status,"cancelled") { return Err(format!("Invalid action transition: {} -> cancelled",current.status)); }
    let now=Utc::now().to_rfc3339();
    db.execute("UPDATE actions SET status='cancelled',updated_at=?1 WHERE id=?2",params![now,id]).map_err(|e|e.to_string())?;
    get_action(&db,&id)
}


#[tauri::command]
pub fn generate_action_draft(app:tauri::AppHandle, action_id:String)->Result<crate::ai::DraftResponse,String>{
    let db=conn(&app)?;
    let action=get_action(&db,&action_id)?;
    if action.status!="requested" && action.status!="prepared" {
        return Err(format!("Action cannot be drafted from status {}",action.status));
    }
    if action.entity_type!="application" { return Err("Draft generation currently requires an application action".into()); }
    let context=crate::context::application_context(&db,&action.entity_id)?;
    let request=crate::ai::DraftRequest{purpose:action.action_type.clone(),context};
    let draft=crate::ai::provider().generate_draft(&request)?;
    let now=Utc::now().to_rfc3339();
    let payload=serde_json::json!({"message":draft.content.clone(),"provider":draft.provider.clone()}).to_string();
    db.execute("UPDATE actions SET status='prepared',payload=?1,updated_at=?2 WHERE id=?3",params![payload,now,action_id]).map_err(|e|e.to_string())?;
    Ok(draft)
}
