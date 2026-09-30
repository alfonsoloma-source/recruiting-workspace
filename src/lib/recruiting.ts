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

export const recruiting = {
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
