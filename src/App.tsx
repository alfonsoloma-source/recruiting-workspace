import { CalendarDays, Plug, Search, Sparkles, UsersRound, BriefcaseBusiness } from "lucide-react";

const attention = [
  { name: "Isabela Domínguez", role: "Product Manager · Finalista", type: "Seguimiento vencido", detail: "7 días sin contacto", action: "Preparar seguimiento" },
  { name: "Javier Ponce", role: "DevOps Engineer · Entrevista", type: "Entrevista por reagendar", detail: "Cambio solicitado", action: "Buscar horario" },
  { name: "Sofía Marín", role: "UX Designer · Técnica", type: "Feedback pendiente", detail: "4 días esperando", action: "Revisar feedback" }
];

export default function App() {
  return <div className="app">
    <aside>
      <div className="brand"><span className="mark">RW</span><div>Recruiting<br/>Workspace</div></div>
      <nav>
        <button className="active"><Search size={17}/>Inicio</button>
        <button><BriefcaseBusiness size={17}/>Vacantes</button>
        <button><UsersRound size={17}/>Candidatos</button>
        <button><CalendarDays size={17}/>Agenda</button>
      </nav>
      <div className="asideBottom"><button><Plug size={17}/>Conexiones</button><div className="profile"><span>CR</span><div><strong>Carla Ríos</strong><small>Recruiter</small></div></div></div>
    </aside>
    <main>
      <header><div><p className="eyebrow">MARTES · WORKSPACE LOCAL</p><h1>Buenos días, Carla.</h1><h2>¿Qué necesita tu atención hoy?</h2></div><button className="assistant"><Sparkles size={16}/>Asistente</button></header>
      <div className="summary"><span><b>3</b> pendientes</span><span><b>2</b> entrevistas hoy</span><span><b>4</b> nuevos</span></div>
      <section><div className="sectionTitle"><span>PRIORIDAD</span><small>3 elementos</small></div><div className="priority">
        {attention.map(item=><article key={item.name}><div className="candidate"><div className="avatar">{item.name.split(" ").map(x=>x[0]).slice(0,2).join("")}</div><div><strong>{item.name}</strong><p>{item.role}</p></div></div><div className="reason"><b>{item.type}</b><span>{item.detail}</span></div><div className="actions"><button>Ver candidato</button><button className="primary">{item.action}</button></div></article>)}
      </div></section>
      <div className="lower"><section><div className="sectionTitle"><span>HOY</span></div><div className="simple"><b>11:00 · Ana Ibáñez</b><span>Entrevista · Product Designer</span></div><div className="simple"><b>15:00 · Luis Torres</b><span>Entrevista · Backend Engineer</span></div></section><section><div className="sectionTitle"><span>NUEVOS POR REVISAR</span></div><div className="simple"><b>María Fernández</b><span>Product Manager · hace 2 h</span></div><div className="simple"><b>Diego Restrepo</b><span>DevOps Engineer · hace 4 h</span></div></section></div>
    </main>
  </div>;
}
