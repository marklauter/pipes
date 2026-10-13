# Pipes docs

Pipes is a service for routing digital artifacts between external systems. It is modeled on a cross-dock warehouse, where goods arrive at in-side doors, a sorter diverts them, and they leave through out-side doors without being stored. In pipes, digital artifacts arrive through doors such as FTP/SFTP servers, mailboxes, web upload endpoints, and watched folders, and the sorter sends each one by the sort plan to one or more out-side doors.

## Notes

Notes record design thinking that is still forming. Each one can become a spec once its open questions are settled.

- [Roadmap](notes/roadmap.md): the build order, from the core through the plugin pipeline, and the build-vs-buy comparison with Apache NiFi.
- [Pipes is a virtual warehouse with in-side and out-side doors](notes/pipes-is-a-virtual-warehouse-with-in-side-and-out-side-doors.md): the cross-dock model, and how doors map to ports and adapters.
- [Events](notes/events.md): the structured events pipes emits for each state change, taps for a live UI, and options for append-only storage.
- [Email adapter reads IMAP behind a trait and tests against GreenMail](notes/email-adapter-reads-imap-behind-a-trait-and-tests-against-greenmail.md): how the mailbox door reads mail, and how it is tested.

## Specs

Specs define how a part of pipes behaves. Implementation follows them.

- [Config](specs/config.md): the three TOML files that define the in-side doors, the out-side doors, and the sort plan, and the rules that validate them.

## Glossary

The [glossary](glossary/) holds one file per term, such as door, route, and digital artifact. Notes and specs use each term only in its glossary meaning.
