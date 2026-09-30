# Recruiting Workspace — Technical Specification v0.1

## 1. Goal

Build a desktop, open-source recruiting workspace that can operate with its own local candidate/job data and optionally connect external communication, calendar, document, ATS and AI providers.

The application core must not depend on Gmail, Google Calendar, a specific ATS, a specific AI provider, or MCP.

## 2. Architectural boundaries

```text
UI
├── Home
├── Jobs
├── Candidates
├── Agenda
├── Connections
└── Contextual Assistant
        │
        ▼
Application / Tool Registry
        │
   ┌────┼──────────────┐
   ▼    ▼              ▼
 Core   AI         MCP adapter
   │
   ▼
Connector contracts
   │
   ├── Communication
   ├── Calendar
   ├── Documents
   └── External candidate/ATS data
```

Provider-specific behavior belongs in adapters/connectors, not in product-domain components.

## 3. Core entities

### Candidate
- id
- name
- email
- phone
- source
- created_at
- updated_at

### Job
- id
- title
- description
- status
- created_at
- updated_at

### Application
Represents a candidate in a job. Candidate and Application are deliberately separate because one person may participate in multiple vacancies.

- id
- candidate_id
- job_id
- stage
- status
- applied_at
- updated_at

### Interview
- id
- application_id
- starts_at
- ends_at
- status
- provider
- external_event_id

### Note
- id
- candidate_id
- application_id (optional)
- content
- created_at
- updated_at

### Action
- id
- type
- entity_type
- entity_id
- status
- due_at
- requires_confirmation
- payload
- created_at
- updated_at

## 4. Action lifecycle

Consequential actions must not be executable merely because an AI model requested them.

```text
requested -> prepared -> awaiting_confirmation -> executing -> completed
                                           \-> cancelled
                                      executing -> failed
```

Examples requiring confirmation in v0.1:
- sending a message/email
- scheduling or rescheduling an interview
- modifying external data
- moving a candidate stage when configured as an external write

Read-only queries may execute without per-action confirmation after the user grants the connector permission.

## 5. Tool layer

Tools describe recruiter capabilities, not vendors.

Initial contracts:

- search_conversations
- get_conversation
- prepare_followup
- send_message
- get_availability
- schedule_interview
- reschedule_interview
- search_documents
- get_document
- list_candidates
- get_candidate
- update_candidate_stage
- add_candidate_note
- list_jobs
- get_job
- get_job_attention_items

The UI, contextual assistant and future MCP adapter should call the same tool registry.

## 6. Connectors

A connector declares:

- manifest
- capabilities
- authentication requirements
- read/write permissions
- health/status
- provider adapters

Example:

```text
google-gmail
capabilities:
  - search_conversations
  - get_conversation
  - send_message
permissions:
  - read
  - write
```

The UI should display recruiter-oriented capabilities such as “Buscar conversaciones” and “Enviar mensaje”; provider/API implementation details remain behind the connector.

## 7. AI provider boundary

AI is optional. Core recruiting workflows must remain usable without an AI provider.

Provider interface should support:
- generate/respond
- tool-use capability metadata
- model selection
- connection health

Planned providers may include OpenAI, Anthropic, Gemini and local models. v0.1 does not require implementing all providers.

API keys/tokens must not be stored as plaintext application records. Desktop credentials should use an appropriate OS-backed secure storage mechanism.

## 8. Contextual assistant

Context may include:
- current candidate
- current application
- current vacancy
- current screen
- selected relevant records

The UI must make active context visible and removable.

Queries can return directly. Consequential actions produce a prepared Action and require confirmation.

## 9. Local-first data

Candidates, vacancies, applications, notes and actions can exist without an external ATS.

SQLite is the planned local persistence layer.

External connectors may import or synchronize data later. The UI must not require an external ATS.

“Local-first” does not mean no data ever leaves the device: external AI and connectors may receive explicitly necessary information according to granted permissions.

## 10. MCP

MCP is not the domain core.

After tool contracts are stable, an MCP server/adapter may expose selected tools from the same registry used by the desktop UI and assistant.

## 11. MVP milestones

### M1 — Local workspace
Desktop shell, local DB, jobs, candidates, applications, notes, actions and agenda with demo/local data.

### M2 — Contextual AI
One real AI provider, contextual candidate/job queries, prepared actions and confirmation flow.

### M3 — Google vertical
Google OAuth, Gmail and Calendar: read conversations, availability, follow-up sending after confirmation, interview creation after confirmation.

### M4 — MCP
Expose mature read/action tools through MCP while preserving permission and confirmation rules.

### M5 — v0.1.0
Windows installer, documentation, demo data, connector authoring guide and first public release.

## 12. MVP acceptance flow

A recruiter can:
1. See that a candidate has gone more than N days without contact.
2. Open the candidate and preserve vacancy context.
3. Prepare a follow-up.
4. Review/edit the generated message.
5. Explicitly confirm.
6. Send through a configured connector.
7. Record the interaction.
8. Remove/update the attention item.

This flow is a stronger MVP acceptance criterion than visual parity alone.
