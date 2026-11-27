# Contributing

Thanks for your interest in koshr. Please read the [Code of Conduct](CODE_OF_CONDUCT.md) first,
and report security problems privately as described in [SECURITY.md](SECURITY.md), never in a
public issue or pull request.

Propose a change with a pull request, and keep it focused. For a large change, describe the
problem and your approach first so a maintainer can say early whether it fits.

## What a good change looks like

- State **why** the change exists, in the pull request description.
- Every behaviour you add or change has a **test**, and the test passes.
- **No personal data.** koshr holds none. Do not add real names, email addresses, profile
  photos or local absolute paths to code, fixtures, docs or commit messages. Test fixtures are
  synthetic.
- **Test first.** Write the test, see it fail, then make it pass.
- **No C.** koshr is pure Rust: no C or C++ code, and no dependency that builds it.
- Dependencies are permissively licensed and pinned.

## Building and checking

You need the Rust toolchain from `rust-toolchain.toml` (via rustup). `make help` lists the
targets. Before opening a pull request, run what CI runs:

```sh
make ci
```

## Wording: agents and people

koshr's product text is written so that contributors using any agent framework are at home. Keep
to these rules in the README, the docs, the interface and the command and API messages:

- Say **agent** (or "your agent") for the autonomous tool, and **the person** or **you** for the
  human.
- Name a framework only in its adapter files: the configuration files, hooks or generated
  instruction blocks that belong to that one framework, and nowhere in product text.
- Stored values and wire fields are not renamed without a compatibility plan.
- The archtests in `crates/koshr-archtest` fail on a vendor agent name in product text outside
  the adapter files. They run in CI, so check them before you open a pull request.

## Branches and commits

Work on one branch per item. Commits are small, say what changed and why, and name their board
item (for example `FTR-012` or `DEF-003`) in the message.

## Licence of contributions

koshr is licensed under MIT OR Apache-2.0. Unless you explicitly state otherwise, any
contribution intentionally submitted for inclusion in koshr by you, as defined in the Apache-2.0
licence, is dual licensed as above, without any additional terms or conditions.

The maintainers in [CONTRIBUTORS](CONTRIBUTORS) review every pull request.
