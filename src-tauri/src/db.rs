use chrono::{Duration, Utc};
use rusqlite::{params, Connection, Result};
use std::fs;
use std::path::PathBuf;

pub fn database_path(app_data_dir: PathBuf) -> PathBuf {
    app_data_dir.join("recruiting-workspace.sqlite3")
}

pub fn open(app_data_dir: PathBuf) -> Result<Connection> {
    fs::create_dir_all(&app_data_dir).map_err(|_| rusqlite::Error::InvalidPath(app_data_dir.clone()))?;
    let conn = Connection::open(database_path(app_data_dir))?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
    migrate(&conn)?;
    seed_demo_if_empty(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<()> {
    conn.execute_batch(include_str!("../migrations/0001_initial.sql"))?;
    Ok(())
}

fn seed_demo_if_empty(conn: &Connection) -> Result<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM candidates", [], |row| row.get(0))?;
    if count > 0 { return Ok(()); }

    let now = Utc::now();
    let seven_days_ago = (now - Duration::days(7)).to_rfc3339();
    let four_days_ago = (now - Duration::days(4)).to_rfc3339();
    let now_s = now.to_rfc3339();

    let candidates = [
        ("demo-candidate-isabela", "Isabela Domínguez", "isabela@example.test"),
        ("demo-candidate-javier", "Javier Ponce", "javier@example.test"),
        ("demo-candidate-sofia", "Sofía Marín", "sofia@example.test"),
        ("demo-candidate-maria", "María Fernández", "maria@example.test"),
    ];
    for (id,name,email) in candidates {
        conn.execute("INSERT INTO candidates(id,name,email,source,created_at,updated_at) VALUES(?1,?2,?3,'Demo',?4,?4)", params![id,name,email,now_s])?;
    }

    conn.execute("INSERT INTO jobs(id,title,description,status,created_at,updated_at) VALUES('demo-job-pm','Product Manager','Demo vacancy','open',?1,?1)", params![now_s])?;
    conn.execute("INSERT INTO jobs(id,title,description,status,created_at,updated_at) VALUES('demo-job-devops','DevOps Engineer','Demo vacancy','open',?1,?1)", params![now_s])?;
    conn.execute("INSERT INTO jobs(id,title,description,status,created_at,updated_at) VALUES('demo-job-ux','UX Designer','Demo vacancy','open',?1,?1)", params![now_s])?;

    conn.execute("INSERT INTO applications(id,candidate_id,job_id,stage,status,applied_at,updated_at) VALUES('demo-app-isabela','demo-candidate-isabela','demo-job-pm','Finalista','active',?1,?1)", params![seven_days_ago])?;
    conn.execute("INSERT INTO applications(id,candidate_id,job_id,stage,status,applied_at,updated_at) VALUES('demo-app-javier','demo-candidate-javier','demo-job-devops','Entrevista','active',?1,?1)", params![now_s])?;
    conn.execute("INSERT INTO applications(id,candidate_id,job_id,stage,status,applied_at,updated_at) VALUES('demo-app-sofia','demo-candidate-sofia','demo-job-ux','Técnica','active',?1,?1)", params![four_days_ago])?;
    conn.execute("INSERT INTO applications(id,candidate_id,job_id,stage,status,applied_at,updated_at) VALUES('demo-app-maria','demo-candidate-maria','demo-job-pm','Nuevo','active',?1,?1)", params![now_s])?;

    conn.execute("INSERT INTO actions(id,type,entity_type,entity_id,status,due_at,requires_confirmation,payload,created_at,updated_at) VALUES('demo-action-isabela','follow_up','application','demo-app-isabela','requested',?1,1,'{}',?2,?2)", params![seven_days_ago,now_s])?;

    let interview_start = (now + Duration::hours(2)).to_rfc3339();
    let interview_end = (now + Duration::hours(3)).to_rfc3339();
    conn.execute("INSERT INTO interviews(id,application_id,starts_at,ends_at,status,provider) VALUES('demo-interview-javier','demo-app-javier',?1,?2,'scheduled','Local')", params![interview_start,interview_end])?;
    Ok(())
}
