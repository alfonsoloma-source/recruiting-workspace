# Contributing

Recruiting Workspace is in early development.

## Principles

Before adding a feature, check that it:
- solves a recruiter workflow rather than adding dashboard surface area;
- keeps provider-specific logic behind adapters;
- does not make AI mandatory;
- preserves explicit confirmation for consequential actions;
- can work with the local data model when an external ATS is absent.

## Early contribution areas

The first phase focuses on core domain contracts, the tool registry, desktop shell and local persistence. Connector APIs should be treated as unstable until documented as stable.

## Pull requests

Keep changes focused. Explain the recruiter problem being solved, architectural impact, and how the change was tested.

Do not commit credentials, OAuth secrets, API keys, candidate personal data or production exports.
