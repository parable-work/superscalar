## What and why

<!-- One paragraph. Link the issue if there is one. -->

## Checklist

- [ ] Every commit is signed off (`git commit -s`); see CONTRIBUTING.md
- [ ] Conformance vectors added or unchanged (if changed: CHANGELOG.md entry
      and version bump per CONTRIBUTING.md)
- [ ] Scalar ids are append-only; nothing renumbered
- [ ] Generated files were regenerated, not hand-edited (`make codegen-check`)
- [ ] `make bindings` passes locally
- [ ] Docs updated where behaviour or API changed
