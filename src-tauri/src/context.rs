use rusqlite::Connection;
use crate::ai::DraftContext;

pub fn application_context(db:&Connection, application_id:&str) -> Result<DraftContext,String> {
    let (candidate_id,candidate_name,job_title,stage):(String,String,String,String)=db.query_row(
        "SELECT c.id,c.name,j.title,a.stage FROM applications a JOIN candidates c ON c.id=a.candidate_id JOIN jobs j ON j.id=a.job_id WHERE a.id=?1",
        [application_id], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))
    ).map_err(|e|e.to_string())?;
    let mut stmt=db.prepare("SELECT content FROM notes WHERE candidate_id=?1 ORDER BY created_at DESC LIMIT 5").map_err(|e|e.to_string())?;
    let notes=stmt.query_map([candidate_id],|r|r.get::<_,String>(0)).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
    Ok(DraftContext{candidate_name,job_title,stage,notes})
}
