import { useEffect, useState } from "react";
import { CalendarDays, Plug, Search, Sparkles, UsersRound, BriefcaseBusiness, ArrowLeft } from "lucide-react";
import { recruiting, CandidateWorkspace, JobWorkspace, HomeWorkspace, InterviewWorkspace } from "./lib/recruiting";

type View = "home" | "candidates" | "candidate" | "jobs" | "job" | "agenda" | "connections";



export default function App() {
  const [view,setView]=useState<View>("home");
  const [rows,setRows]=useState<CandidateWorkspace[]>([]);
  const [selected,setSelected]=useState<CandidateWorkspace|null>(null);
  const [loading,setLoading]=useState(false);
  const [jobs,setJobs]=useState<JobWorkspace[]>([]);
  const [selectedJob,setSelectedJob]=useState<JobWorkspace|null>(null);
  const [jobRows,setJobRows]=useState<CandidateWorkspace[]>([]);
  const [home,setHome]=useState<HomeWorkspace|null>(null);
  const [interviews,setInterviews]=useState<InterviewWorkspace[]>([]);
  const [draftAction,setDraftAction]=useState<{id:string;candidate:string;job:string;message:string;status:string}|null>(null);

  async function loadCandidates(){
    setLoading(true);
    try { setRows(await recruiting.listCandidateWorkspace()); }
    finally { setLoading(false); }
  }
  useEffect(()=>{ if(view==="home") recruiting.getHomeWorkspace().then(setHome); if(view==="candidates") loadCandidates(); if(view==="jobs") recruiting.listJobWorkspace().then(setJobs); if(view==="agenda") recruiting.listInterviews().then(setInterviews); },[view]);

  async function openJob(job:JobWorkspace){ setSelectedJob(job); setJobRows(await recruiting.listJobApplications(job.job_id)); setView("job"); }

  function openCandidate(row:CandidateWorkspace){ setSelected(row); setView("candidate"); }

  return <div className="app">
    <aside>
      <div className="brand"><span className="mark">RW</span><div>Recruiting<br/>Workspace</div></div>
      <nav>
        <button className={view==="home"?"active":""} onClick={()=>setView("home")}><Search size={17}/>Inicio</button>
        <button className={view==="jobs"||view==="job"?"active":""} onClick={()=>setView("jobs")}><BriefcaseBusiness size={17}/>Vacantes</button>
        <button className={view==="candidates"||view==="candidate"?"active":""} onClick={()=>setView("candidates")}><UsersRound size={17}/>Candidatos</button>
        <button className={view==="agenda"?"active":""} onClick={()=>setView("agenda")}><CalendarDays size={17}/>Agenda</button>
      </nav>
      <div className="asideBottom"><button onClick={()=>setView("connections")}><Plug size={17}/>Conexiones</button><div className="profile"><span>CR</span><div><strong>Carla Ríos</strong><small>Recruiter</small></div></div></div>
    </aside>
    <main>
      {view==="home" && <Home data={home} openCandidate={openCandidate} prepareFollowup={async(item)=>{const draft=await recruiting.generateActionDraft(item.action_id);setDraftAction({id:item.action_id,candidate:item.candidate_name,job:item.job_title,message:draft.content,status:"prepared"});}}/>}
      {view==="candidates" && <Candidates rows={rows} loading={loading} openCandidate={openCandidate}/>}
      {view==="candidate" && selected && <CandidateDetail row={selected} back={()=>setView("candidates")} refresh={loadCandidates}/>}
      {view==="jobs" && <Jobs jobs={jobs} openJob={openJob}/>}
      {view==="job" && selectedJob && <JobDetail job={selectedJob} rows={jobRows} back={()=>setView("jobs")} openCandidate={openCandidate}/>}
      {view==="agenda" && <Agenda interviews={interviews} openCandidate={openCandidate}/>}
      {view==="connections" && <Placeholder title="Conexiones" text="Tus herramientas, permisos y proveedores vivirán aquí."/>}
      {draftAction && <ActionComposer draft={draftAction} setDraft={setDraftAction} close={()=>setDraftAction(null)} refreshHome={()=>recruiting.getHomeWorkspace().then(setHome)}/>}
    </main>
  </div>;
}

function Home({data,openCandidate,prepareFollowup}:{data:HomeWorkspace|null;openCandidate:(r:CandidateWorkspace)=>void;prepareFollowup:(i:HomeWorkspace["attention"][number])=>Promise<void>}){
 const label=(type:string)=>type==="follow_up"?"Seguimiento pendiente":type.replaceAll("_"," ");
 return <><header><div><p className="eyebrow">WORKSPACE LOCAL</p><h1>Buenos días, Carla.</h1><h2>¿Qué necesita tu atención hoy?</h2></div><button className="assistant"><Sparkles size={16}/>Asistente</button></header>
 <div className="summary"><span><b>{data?.pending_count??"—"}</b> pendientes</span><span><b>{data?.interview_today_count??"—"}</b> entrevistas hoy</span><span><b>{data?.new_count??"—"}</b> nuevos</span></div>
 <section><div className="sectionTitle"><span>PRIORIDAD</span><small>{data?.attention.length??0} elementos</small></div><div className="priority">
 {(data?.attention||[]).map(item=><article key={item.action_id}><div className="candidate"><div className="avatar">{initials(item.candidate_name)}</div><div><strong>{item.candidate_name}</strong><p>{item.job_title} · {item.stage}</p></div></div><div className="reason"><b>{label(item.action_type)}</b><span>{item.due_at?new Date(item.due_at).toLocaleDateString():"Sin fecha límite"}</span></div><div className="actions"><button onClick={()=>openCandidate({application_id:item.application_id,candidate_id:item.candidate_id,candidate_name:item.candidate_name,email:null,job_id:"",job_title:item.job_title,stage:item.stage,status:"active",updated_at:item.due_at||""})}>Ver candidato</button><button className="primary" onClick={()=>prepareFollowup(item)}>{item.action_type==="follow_up"?"Preparar seguimiento":"Revisar"}</button></div></article>)}
 {!data?.attention.length&&<p className="emptyState">Nada requiere atención por ahora.</p>}</div></section>
 <div className="lower"><section><div className="sectionTitle"><span>HOY</span></div><div className="simple"><b>{data?.interview_today_count||0} entrevistas</b><span>La agenda detallada se conecta en el siguiente bloque.</span></div></section><section><div className="sectionTitle"><span>NUEVOS POR REVISAR</span></div>{(data?.new_candidates||[]).map(r=><button className="newCandidate" key={r.application_id} onClick={()=>openCandidate(r)}><b>{r.candidate_name}</b><span>{r.job_title}</span></button>)}</section></div></>;
}



function Candidates({rows,loading,openCandidate}:{rows:CandidateWorkspace[];loading:boolean;openCandidate:(r:CandidateWorkspace)=>void}){
  return <><PageHeader eyebrow="CANDIDATOS" title="Candidatos" subtitle="Procesos activos en tu workspace."/>
    <section className="panel">{loading?<p className="emptyState">Cargando…</p>:rows.map(r=><button className="listRow" key={r.application_id} onClick={()=>openCandidate(r)}><div><strong>{r.candidate_name}</strong><span>{r.job_title}</span></div><span>{r.stage}</span></button>)}
    {!loading&&!rows.length&&<p className="emptyState">Todavía no hay candidatos.</p>}</section></>;
}

function CandidateDetail({row,back,refresh}:{row:CandidateWorkspace;back:()=>void;refresh:()=>Promise<void>}){
  const [stage,setStage]=useState(row.stage);
  const [note,setNote]=useState("");
  const [saving,setSaving]=useState(false);
  async function saveStage(){setSaving(true);try{await recruiting.updateApplicationStage(row.application_id,stage);await refresh();}finally{setSaving(false);}}
  async function addNote(){if(!note.trim())return;setSaving(true);try{await recruiting.addCandidateNote({candidate_id:row.candidate_id,application_id:row.application_id,content:note.trim()});setNote("");}finally{setSaving(false);}}
  return <><button className="back" onClick={back}><ArrowLeft size={16}/>Candidatos</button><PageHeader eyebrow={row.job_title} title={row.candidate_name} subtitle={row.email||"Sin correo registrado"}/>
    <div className="detailGrid"><section className="panel"><div className="sectionTitle"><span>PROCESO</span></div><label>Etapa</label><div className="inlineForm"><input value={stage} onChange={e=>setStage(e.target.value)}/><button className="primary" disabled={saving} onClick={saveStage}>Guardar etapa</button></div></section>
    <section className="panel"><div className="sectionTitle"><span>NOTA</span></div><textarea value={note} onChange={e=>setNote(e.target.value)} placeholder="Añade contexto del candidato…"/><button className="primary" disabled={saving||!note.trim()} onClick={addNote}>Agregar nota</button></section></div></>;
}

function Jobs({jobs,openJob}:{jobs:JobWorkspace[];openJob:(j:JobWorkspace)=>void}){
 return <><PageHeader eyebrow="VACANTES" title="Vacantes" subtitle="Pipeline y atención por vacante."/><section className="panel">{jobs.map(j=><button className="listRow" key={j.job_id} onClick={()=>openJob(j)}><div><strong>{j.title}</strong><span>{j.total} candidatos · {j.status}</span></div><span>{j.finalist_count} finalistas</span></button>)}{!jobs.length&&<p className="emptyState">Todavía no hay vacantes.</p>}</section></>;
}

function JobDetail({job,rows,back,openCandidate}:{job:JobWorkspace;rows:CandidateWorkspace[];back:()=>void;openCandidate:(r:CandidateWorkspace)=>void}){
 return <><button className="back" onClick={back}><ArrowLeft size={16}/>Vacantes</button><PageHeader eyebrow="VACANTE" title={job.title} subtitle={job.total+" candidatos activos"}/>
 <div className="summary"><span><b>{job.new_count}</b> nuevos</span><span><b>{job.screening_count}</b> screening</span><span><b>{job.interview_count}</b> entrevista</span><span><b>{job.finalist_count}</b> finalistas</span></div>
 <section className="panel">{rows.map(r=><button className="listRow" key={r.application_id} onClick={()=>openCandidate(r)}><div><strong>{r.candidate_name}</strong><span>{r.email||"Sin correo"}</span></div><span>{r.stage}</span></button>)}</section></>;
}

function Agenda({interviews,openCandidate}:{interviews:InterviewWorkspace[];openCandidate:(r:CandidateWorkspace)=>void}){
 return <><PageHeader eyebrow="AGENDA" title="Entrevistas" subtitle="Solo eventos relacionados con tus procesos de selección."/><section className="panel">{interviews.map(i=><button className="listRow" key={i.interview_id} onClick={()=>openCandidate({application_id:i.application_id,candidate_id:i.candidate_id,candidate_name:i.candidate_name,email:null,job_id:"",job_title:i.job_title,stage:i.stage,status:"active",updated_at:i.starts_at})}><div><strong>{i.candidate_name}</strong><span>{i.job_title} · {new Date(i.starts_at).toLocaleString()}</span></div><span>{i.status}</span></button>)}{!interviews.length&&<p className="emptyState">No hay entrevistas programadas.</p>}</section></>;
}

function Placeholder({title,text}:{title:string;text:string}){return <><PageHeader eyebrow="WORKSPACE" title={title} subtitle={text}/><section className="panel"><p className="emptyState">Esta sección se habilitará por capacidades, sin obligar al recruiter a configurar herramientas técnicas.</p></section></>}

function ActionComposer({draft,setDraft,close,refreshHome}:{draft:{id:string;candidate:string;job:string;message:string;status:string};setDraft:React.Dispatch<React.SetStateAction<{id:string;candidate:string;job:string;message:string;status:string}|null>>;close:()=>void;refreshHome:()=>void}){
 const [busy,setBusy]=useState(false); const [error,setError]=useState("");
 async function requestConfirmation(){if(!draft.message.trim())return;setBusy(true);setError("");try{await recruiting.prepareAction(draft.id,{message:draft.message.trim()});await recruiting.requestActionConfirmation(draft.id);setDraft({...draft,message:draft.message.trim(),status:"awaiting_confirmation"});await refreshHome();}catch(e){setError(String(e));}finally{setBusy(false);}}
 async function confirm(){setBusy(true);setError("");try{await recruiting.confirmAction(draft.id);setDraft({...draft,status:"executing"});await refreshHome();}catch(e){setError(String(e));}finally{setBusy(false);}}
 async function complete(){setBusy(true);setError("");try{await recruiting.completeAction(draft.id);await refreshHome();close();}catch(e){setError(String(e));}finally{setBusy(false);}}
 return <div className="composerBackdrop"><div className="composer"><div className="sectionTitle"><span>SEGUIMIENTO</span><button onClick={close}>Cerrar</button></div><h3>{draft.candidate}</h3><p>{draft.job}</p><textarea value={draft.message} disabled={draft.status!=="prepared"} onChange={e=>setDraft({...draft,message:e.target.value})}/>
 {error&&<p className="errorText">{error}</p>}
 {draft.status==="prepared"&&<button className="primary" disabled={busy||!draft.message.trim()} onClick={requestConfirmation}>Solicitar confirmación</button>}
 {draft.status==="awaiting_confirmation"&&<><p className="confirmText">Revisa el mensaje. Nada se ejecuta hasta que confirmes.</p><button className="primary" disabled={busy} onClick={confirm}>Confirmar acción</button></>}
 {draft.status==="executing"&&<><p className="confirmText">La acción está confirmada. Todavía no existe un conector externo, así que esta etapa solo valida el flujo interno.</p><button className="primary" disabled={busy} onClick={complete}>Completar simulación</button></>}</div></div>;
}

function PageHeader({eyebrow,title,subtitle}:{eyebrow:string;title:string;subtitle:string}){return <header><div><p className="eyebrow">{eyebrow}</p><h1>{title}</h1><h2>{subtitle}</h2></div><button className="assistant"><Sparkles size={16}/>Asistente</button></header>}
function initials(name:string){return name.split(/\s+/).filter(Boolean).slice(0,2).map(x=>x[0]?.toUpperCase()).join("");}
