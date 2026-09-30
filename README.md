# Recruiting Workspace

Open-source, local-first workspace for recruiters.

Recruiting Workspace brings vacancies, candidates, interviews, follow-ups and contextual AI into one desktop workspace. The product is designed around a simple question:

> What needs my attention today?

## Status

**v0.1 — early development.** The UX has been prototyped and the technical foundation is now being built.

## Product principles

- Actions before metrics.
- Recruiter language instead of provider/API language.
- Local-first data ownership where practical.
- AI is optional and contextual, not the whole interface.
- Human confirmation before consequential external actions.
- Providers are replaceable behind common capabilities.
- Connectors are modular and community-extensible.
- MCP is an interface over mature tools, not the application core.

## Planned architecture

- **Desktop:** Tauri
- **UI:** React + TypeScript
- **Local data:** SQLite
- **AI:** provider adapters / BYOK
- **Integrations:** capability-based connectors
- **MCP:** adapter over the shared tool registry

## MVP

The first end-to-end workflow is:

1. Surface a candidate who needs follow-up.
2. Open the candidate with vacancy context.
3. Prepare a follow-up with AI.
4. Let the recruiter edit or confirm it.
5. Send through the configured communication connector.
6. Record the interaction locally.
7. Clear the attention item.

## Repository structure

```text
apps/
  desktop/
packages/
  core/
  tools/
  ai/
  connectors/
docs/
  architecture/
  product/
```

See [docs/architecture/technical-spec-v0.1.md](docs/architecture/technical-spec-v0.1.md) for the initial technical specification.

## Contributing

The project is at an early stage. Connector and core contracts will be stabilized before community integrations are accepted. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
