## Task and requirements

<!--
What was asked, and what "done" means for it. Name the consuming crate or application when
the change exists for one of them.
-->

## What changed

<!--
The pieces, in the order a reviewer should read them. Say whether the public API changed.
-->

## Tests

<!--
Which tests prove the behaviour, and why they would fail without this change. For a shared
crate, say which consumer was verified when the API changed.
-->

## Checks run

- [ ] `cargo xtask ci` (exit 0)
- [ ] The two consumers still build: `ankou` and `lpdnsoft-launcher` (path dependencies)
- [ ] `CHANGELOG`/version bumped when the public API changed

## What is not done

<!--
Known gaps, each with the reason. "None" is acceptable only when it is true.
-->

## Checklist

- [ ] A public API change is documented on the item, with a `# Examples` section when useful
- [ ] No `unsafe`, and clippy stays clean with `-D warnings`
- [ ] Rust 1.90 still compiles the crates (1.95 for `lpdnsoft-ui`, which follows egui 0.36)
- [ ] Commits are signed off (`git commit -s`) and carry no tool attribution
