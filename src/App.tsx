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
  const [home,setHome]=useState<HomeWorkspace|null>(null);\n  const [interviews,setInterviews]=useState<InterviewWorkspace[]>([]);\n  const [draftAction,setDraftAction]=useState<{id:string;candidate:string;job:string;message:string;status:string}|null>(null);

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
      {view==="home" && <Home data={home} openCandidate={openCandidate} prepareFollowup={async(item)=>{const action=await recruiting.prepareAction(item.action_id,{message:""});setDraftAction({id:action.id,candidate:item.candidate_name,job:item.job_title,message:"",status:action.status});}}/>}
      {view==="candidates" && <Candidates rows={rows} loading={loading} openCandidate={openCandidate}/>}
      {view==="candidate" && selected && <CandidateDetail row={selected} back={()=>setView("candidates")} refresh={loadCandidates}/>}
      {view==="jobs" && <Jobs jobs={jobs} openJob={openJob}/>}
      {view==="job" && selectedJob && <JobDetail job={selectedJob} rows={jobRows} back={()=>setView("jobs")} openCandidate={openCandidate}/>}
      {view==="agenda" && <Agenda interviews={interviews} openCandidate={openCandidate}/>}
      {view==="connections" && <Placeholder title="Conexiones" text="Tus herramientas, permisos y proveedores vivirán aquí."/>}
      {draftAction && <ActionComposer draft={draftAction} setDraft={setDraftAction} close={()=>setDraftAction(null)} refreshHome={()=>recruiting.getHomeWorkspace().then(setHome)}/>}\n    </main>
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

