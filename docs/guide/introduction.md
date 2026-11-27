# koshr

<div class="brand-lockup">
  <img class="on-light" src="images/brand/koshr-lockup-light.svg" alt="koshr: kośa, a treasury, kept">
  <img class="on-dark" src="images/brand/koshr-lockup-dark.svg" alt="koshr: kośa, a treasury, kept">
</div>

**koshr** is an open-source, self-hostable code store that stays git-compatible for existing
tooling, with verified identity, row-level access control and signed, non-repudiable mutations.

koshr is a tool, not a service. Hosting is offered only through [Provr](https://provr.dev), which
runs released koshr unchanged.

## What it is for

Unmodified git clients and tools clone, fetch and push to koshr. koshr accepts a push only if its
access rules allow it and every commit is signed with a key bound to the pusher's registered
identity. The history it keeps is therefore signed and checkable.

## Status

Version 0.1.0 claims the name: there is little to run yet. Next comes the verify gate
(`koshr verify`: a git server refuses pushes whose commits are not signed by a key bound to a live
Sigillr identity), then `koshr merge`.

See [Installing](installing.md) and [Commands](commands.md).
