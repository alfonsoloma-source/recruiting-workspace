import { invoke } from "@tauri-apps/api/core";

export type Candidate = {
  id: string;
  name: string;
  email?: string | null;
  phone?: string | null;
  source?: string | null;
};

export type Job = {
  id: string;
  title: string;
  description?: string | null;
  status: string;
};

export type Application = {
  id: string;
  candidate_id: string;
  job_id: string;
  stage: string;
  status: string;
};


export type CandidateWorkspace = {
  application_id: string;
  candidate_id: string;
  candidate_name: string;
  email?: string | null;
  job_id: string;
  job_title: string;
  stage: string;
  status: string;
  updated_at: string;
};
\nexport type JobWorkspace = {
  job_id: string;
  title: string;
  status: string;
  total: number;
  new_count: number;
  screening_count: number;
  interview_count: number;
  finalist_count: number;
};

export type AttentionItem = {
  action_id: string;
  application_id: string;
  candidate_id: string;
  candidate_name: string;
  job_title: string;
  stage: string;
  action_type: string;
  due_at?: string | null;
  requires_confirmation: boolean;
};

export type HomeWorkspace = {
  pending_count: number;
  interview_today_count: number;
  new_count: number;
  attention: AttentionItem[];
  new_candidates: CandidateWorkspace[];
};

export type InterviewWorkspace = {
  interview_id: string;
  application_id: string;
  candidate_id: string;
  candidate_name: string;
  job_title: string;
  stage: string;
  starts_at: string;
  ends_at: string;
  status: string;
  provider?: string | null;
};

export type WorkspaceAction = {
  id: string;
  action_type: string;
  entity_type: string;
  entity_id: string;
  status: "requested" | "prepared" | "awaiting_confirmation" | "executing" | "completed" | "cancelled" | "failed";
  due_at?: string | null;
  requires_confirmation: boolean;
  payload: Record<string, unknown>;
  created_at: string;
  updated_at: string;
};

export const recruiting = {
  generateActionDraft: (actionId:string) => invoke<{provider:string;content:string}>("generate_action_draft",{actionId}),
  createAction: (input: { action_type:string; entity_type:string; entity_id:string; due_at?:string|null; requires_confirmation?:boolean; payload?:Record<string,unknown> }) => invoke<WorkspaceAction>("create_action",{input}),
  prepareAction: (id:string,payload:Record<string,unknown>) => invoke<WorkspaceAction>("prepare_action",{id,payload}),
  requestActionConfirmation: (id:string) => invoke<WorkspaceAction>("request_action_confirmation",{id}),
  confirmAction: (id:string) => invoke<WorkspaceAction>("confirm_action",{id}),
  completeAction: (id:string) => invoke<WorkspaceAction>("complete_action",{id}),
  cancelAction: (id:string) => invoke<WorkspaceAction>("cancel_action",{id}),
  getHomeWorkspace: () => invoke<HomeWorkspace>("get_home_workspace"),
  listInterviews: () => invoke<InterviewWorkspace[]>("list_interview_workspace"),
  createInterview: (input: { application_id: string; starts_at: string; ends_at: string; provider?: string | null }) => invoke<string>("create_interview", { input }),
  listCandidates: () => invoke<Candidate[]>("list_candidates"),\n  listCandidateWorkspace: () => invoke<CandidateWorkspace[]>("list_candidate_workspace"),
  createCandidate: (input: Omit<Candidate, "id">) =>
    invoke<Candidate>("create_candidate", { input }),

  listJobs: () => invoke<Job[]>("list_jobs"),
  listJobWorkspace: () => invoke<JobWorkspace[]>("list_job_workspace"),
  listJobApplications: (job_id: string) => invoke<CandidateWorkspace[]>("list_job_applications", { jobId: job_id }),
  createJob: (input: Pick<Job, "title" | "description">) =>
    invoke<Job>("create_job", { input }),

  createApplication: (input: {
    candidate_id: string;
    job_id: string;
    stage?: string;
  }) => invoke<Application>("create_application", { input }),

  updateApplicationStage: (id: string, stage: string) =>
    invoke<void>("update_application_stage", { id, stage }),

  addCandidateNote: (input: {
    candidate_id: string;
    application_id?: string | null;
    content: string;
  }) => invoke<string>("add_candidate_note", { input }),
};
