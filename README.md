# pipes

Pipes is a service for routing digital artifacts between external systems. It follows the cross-docking model: digital artifacts pass straight through and are not stored. Digital artifacts arrive through in-side doors such as FTP/SFTP servers, mailboxes, web upload endpoints, and watched folders. The sorter sends each one, by the sort plan, to depart through one or more out-side doors. Digital artifacts that need a person go to an attention-queue. Each state change emits a structured event, which a UI can tap to show live activity.

Pipes runs on Windows and Linux. The build order is in [docs/notes/roadmap.md](docs/notes/roadmap.md), and the terms are defined in [docs/glossary](docs/glossary).
