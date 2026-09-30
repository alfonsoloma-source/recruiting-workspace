# Security Policy

Recruiting Workspace handles potentially sensitive recruiting data.

Please do not open public issues containing API keys, OAuth tokens, candidate personal data, private resumes, production database exports or other secrets.

For the early development phase, security-sensitive reports should be shared privately with the maintainer rather than posted with exploit details in a public issue.

## Baseline requirements

- Never store provider credentials in plaintext application records.
- Request the minimum connector scopes needed.
- Separate read permissions from write permissions.
- Require explicit user confirmation before consequential external actions.
- Avoid logging candidate content, tokens or secrets unnecessarily.
