<!-- kanbanr:begin — generated, edits here are overwritten -->

## This project is tracked with kanbanr

The board is the system of record for scope, reasoning, progress and documentation. It is a separate git repository beside this one. **Recover from it at the start of every session** and record work there as you go — this block is a pointer, not a copy.

The board is published at <https://github.com/startr-trade/koshr-board>. Clone it beside this repository as `koshr.kanbanr` (`git clone https://github.com/startr-trade/koshr-board ../koshr.kanbanr`) and kanbanr finds it.

**Why this project exists.** koshr is a better git implementation that stays git-compatible for existing tooling: an open-source, self-hostable git server tool with the features of GitHub's and GitLab's software. koshr is a tool, not a service. Hosting and related services are offered only through provr, which runs released koshr unchanged, beside kanbanr, katalogr, Sutra, sutra-voice and honr. It is built on a database-backed distributed code store: centralized truth, cert-leased offline work, verified identity, row-level access control and island-bound, non-repudiable mutations.

**What it commits to** (work items link these by id):

- `G-0` The system stays operable and maintainable
- `G-1` Feature parity with GitHub and GitLab's software
- `G-2` Access is enforced at the data layer, per file, folder and branch (RLS)
- `G-3` Revocation of the native client's local copy is bounded by its certificate's lifetime
- `G-4` Every mutation is non-repudiable and bound to one island
- `G-5` Unmodified git clients and tools clone, fetch, push and open PRs over HTTPS and SSH. Each git
client is served a private view of the repository that koshr maintains from its ACL. A push or
PR is accepted only if koshr's ACL allows it and every commit is signed (SSH or GPG) with a key
bound to the pusher's registered identity. Force-push is refused on protected refs and allowed
elsewhere, and the attested log keeps the replaced head.
- `G-6` The native client commits, syncs and rebases against the island, and rejection is immediate and cryptographic
- `G-7` koshr offers the useful features of Jujutsu and other git variants and alternatives that work without changing git clients
- `G-8` Any koshr serves kanbanr boards and katalogr-indexed repositories through plain git
- `G-9` Pipelines (CI/CD) and agent runs execute as Sutra BPMN flows on runners koshr drives (the
operator's own, or provr's fleet). People write YAML compiled to BPMN, Sutra orchestrates, a
koshr runner executes, and every verdict waits for a person on kanbanr's Review page.
- `G-10` koshr Pages: a repository publishes several sites, from honr or any static builder, at default,
custom, subdomain or path addresses, with versions, previews and visibility from koshr's ACL.

**Deliberately out of scope** — do not propose these as gaps:

- Operating any hosted service (that is provr's alone)
- Force-push on protected refs
- Cross-island merge, or submit once and appear everywhere
- Peer-to-peer or island-to-island replication
- Air-gapped or serverless operation
- SaaS-only code in this repository (tenancy, billing, metering and the hosted fleet live in provr's private repository, which runs released koshr unchanged)

**Constraints:**

- RLS is the sole access-control enforcement point
- The server is authoritative; the native client's local DB is a leased buffer
- Every mutation carries an island-bound attestation
- Storage is Postgres
- MIT OR Apache-2.0 licence
- Sutra is used only through its public, tagged releases (startr-trade/sutra), and koshr work never changes Sutra (payapt depends on it)
- The brand follows the startr.trade family guide (lower-case name, shared palette, Noto Sans, saffron once, the family thread)

**How work moves here** — each stage asks only for what it needs; `kanbanr check <CODE>` names what the next one still lacks:

- **Plan** — Say what it is and why it matters.
- **Design** — What it must do, agreed before it is built.
- **Develop** — Build it — how and where settled, a test named for every requirement.
- **Test** — Test it against its requirements.
- **Review** — Review it with the evidence in hand.
- **Released** — Shipped in a release.

**Before starting an item:** `kanbanr lessons --for <CODE>` — what this project already learned. **Before calling one done:** `kanbanr check <CODE>`.

| Question | Command |
|---|---|
| What is planned, in progress and done? | `kanbanr board` |
| What can I pick up now? | `kanbanr ready` |
| Why does this item exist, and how is it verified? | `kanbanr feature show <CODE>` |
| Why is the architecture like this? | `kanbanr adr list` |
| Why does this line of code exist? | `kanbanr why <file>:<line>` |
| What did the last wave cost? | `kanbanr retro <MS-00x>` |

Regenerate this block with `kanbanr claude sync`.

<!-- kanbanr:end -->
