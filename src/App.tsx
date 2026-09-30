import { useEffect, useState } from "react";
import { CalendarDays, Plug, Search, Sparkles, UsersRound, BriefcaseBusiness, ArrowLeft } from "lucide-react";
import { recruiting, CandidateWorkspace } from "./lib/recruiting";

type View = "home" | "candidates" | "candidate" | "jobs" | "agenda" | "connections";

const attention = [
  { name: "Isabela Domínguez", role: "Product Manager · Finalista", type: "Seguimiento vencido", detail: "7 días sin contacto", action: "Preparar seguimiento" },
  { name: "Javier Ponce", role: "DevOps Engineer · Entrevista", type: "Entrevista por reagendar", detail: "Cambio solicitado", action: "Buscar horario" },
  { name: "Sofía Marín", role: "UX Designer · Técnica", type: "Feedback pendiente", detail: "4 días esperando", action: "Revisar feedback" }
];

export default function App() {
  const [view,setView]=useState<View>("home");
  const [rows,setRows]=useState<CandidateWorkspace[]>([]);
  const [selected,setSelected]=useState<CandidateWorkspace|null>(null);
  const [loading,setLoading]=useState(false);

  async function loadCandidates(){
    setLoading(true);
    try { setRows(await recruiting.listCandidateWorkspace()); }
    finally { setLoading(false); }
  }
  useEffect(()=>{ if(view==="candidates") loadCandidates(); },[view]);

  function openCandidate(row:CandidateWorkspace){ setSelected(row); setView("candidate"); }

  return <div className="app">
    <aside>
      <div className="brand"><span className="mark">RW</span><div>Recruiting<br/>Workspace</div></div>
      <nav>
        <button className={view==="home"?"active":""} onClick={()=>setView("home")}><Search size={17}/>Inicio</button>
        <button className={view==="jobs"?"active":""} onClick={()=>setView("jobs")}><BriefcaseBusiness size={17}/>Vacantes</button>
        <button className={view==="candidates"||view==="candidate"?"active":""} onClick={()=>setView("candidates")}><UsersRound size={17}/>Candidatos</button>
        <button className={view==="agenda"?"active":""} onClick={()=>setView("agenda")}><CalendarDays size={17}/>Agenda</button>
      </nav>
      <div className="asideBottom"><button onClick={()=>setView("connections")}><Plug size={17}/>Conexiones</button><div className="profile"><span>CR</span><div><strong>Carla Ríos</strong><small>Recruiter</small></div></div></div>
    </aside>
    <main>
      {view==="home" && <Home />}
      {view==="candidates" && <Candidates rows={rows} loading={loading} openCandidate={openCandidate}/>}
      {view==="candidate" && selected && <CandidateDetail row={selected} back={()=>setView("candidates")} refresh={loadCandidates}/>}
      {view==="jobs" && <Placeholder title="Vacantes" text="El workspace de vacantes se conecta en el siguiente bloque."/>}
      {view==="agenda" && <Placeholder title="Agenda" text="Aquí aparecerán únicamente entrevistas y disponibilidad relevante."/>}
      {view==="connections" && <Placeholder title="Conexiones" text="Tus herramientas, permisos y proveedores vivirán aquí."/>}
    </main>
  </div>;
}

function Home(){
 return <><header><div><p className="eyebrow">MARTES · WORKSPACE LOCAL</p><h1>Buenos días, Carla.</h1><h2>¿Qué necesita tu atención hoy?</h2></div><button className="assistant"><Sparkles size={16}/>Asistente</button></header>
 <div className="summary"><span><b>3</b> pendientes</span><span><b>2</b> entrevistas hoy</span><span><b>4</b> nuevos</span></div>
 <section><div className="sectionTitle"><span>PRIORIDAD</span><small>3 elementos</small></div><div className="priority">{attention.map(item=><article key={item.name}><div className="candidate"><div className="avatar">{initials(item.name)}</div><div><strong>{item.name}</strong><p>{item.role}</p></div></div><div className="reason"><b>{item.type}</b><span>{item.detail}</span></div><div className="actions"><button>Ver candidato</button><button className="primary">{item.action}</button></div></article>)}</div></section>
 <div className="lower"><section><div className="sectionTitle"><span>HOY</span></div><div className="simple"><b>11:00 · Ana Ibáñez</b><span>Entrevista · Product Designer</span></div><div className="simple"><b>15:00 · Luis Torres</b><span>Entrevista · Backend Engineer</span></div></section><section><div className="sectionTitle"><span>NUEVOS POR REVISAR</span></div><div className="simple"><b>María Fernández</b><span>Product Manager · hace 2 h</span></div></section></div></>;
}

function Candidates({rows,loading,openCandidate}:{rows:CandidateWorkspace[];loading:boolean;openCandidate:(r:CandidateWorkspace)=>void}){
 return <><header><div><p className="eyebrow">CANDIDATOS</p><h1>Tu pipeline, sin ruido.</h1><h2>Personas activas en procesos.</h2></div></header>
 <div className="summary"><span><b>{rows.length}</b> aplicaciones</span></div>
 <div className="candidateList">{loading?<p>Cargando…</p>:rows.map(r=><button className="candidateRow" key={r.application_id} onClick={()=>openCandidate(r)}><div className="candidate"><div className="avatar">{initials(r.candidate_name)}</div><div><strong>{r.candidate_name}</strong><p>{r.email||"Sin correo"}</p></div></div><div><strong>{r.job_title}</strong><p>{r.stage}</p></div><span className="status">{r.status}</span></button>)}</div></>;
}

function CandidateDetail({row,back,refresh}:{row:CandidateWorkspace;back:()=>void;refresh:()=>Promise<void>}){
 const [stage,setStage]=useState(row.stage); const [note,setNote]=useState(""); const [saved,setSaved]=useState("");
 async function saveStage(){ await recruiting.updateApplicationStage(row.application_id,stage); setSaved("Etapa actualizada"); await refresh(); }
 async function saveNote(){ if(!note.trim())return; await recruiting.addCandidateNote({candidate_id:row.candidate_id,application_id:row.application_id,content:note.trim()}); setNote(""); setSaved("Nota guardada"); }
 return <><button className="back" onClick={back}><ArrowLeft size={15}/> Candidatos</button><header><div><p className="eyebrow">{row.job_title.toUpperCase()}</p><h1>{row.candidate_name}</h1><h2>{row.email||"Sin correo registrado"}</h2></div></header>
 <div className="detailGrid"><section className="panel"><div className="sectionTitle"><span>PROCESO</span></div><label>Etapa<select value={stage} onChange={e=>setStage(e.target.value)}><option>Nuevo</option><option>Screening</option><option>Entrevista</option><option>Técnica</option><option>Finalista</option><option>Contratado</option><option>Descartado</option></select></label><button className="save" onClick={saveStage}>Guardar etapa</button></section>
 <section className="panel"><div className="sectionTitle"><span>NOTA</span></div><textarea value={note} onChange={e=>setNote(e.target.value)} placeholder="Agrega contexto para ti o tu equipo…"/><button className="save" onClick={saveNote}>Guardar nota</button></section></div>{saved&&<p className="saved">{saved}</p>}</>;
}

function Placeholder({title,text}:{title:string;text:string}){return <header><div><p className="eyebrow">{title.toUpperCase()}</p><h1>{title}</h1><h2>{text}</h2></div></header>}
function initials(name:string){return name.split(" ").filter(Boolean).map(x=>x[0]).slice(0,2).join("").toUpperCase()}
