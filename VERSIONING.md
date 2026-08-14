# Versioning & Releases

This project follows [SemVer 2.0](https://semver.org/): `MAJOR.MINOR.PATCH[-PRERELEASE]`, tagged as
`vX.Y.Z` (e.g. `v0.3.0`, `v0.3.0-rc.1`).

## When to use what

| Segment | Format | When to use it | Example tag |
|---|---|---|---|
| **Major** | `X.0.0` | Breaking changes -- syntax changes, removed/renamed stdlib functions, incompatible `.rlc` bytecode, anything that breaks existing `.rl` scripts or crates depending on the workspace | `v1.0.0`, `v2.0.0` |
| **Minor** | `x.Y.0` | New features that don't break existing code -- new stdlib module, new language feature that's additive | `v0.3.0`, `v0.4.0` |
| **Patch** | `x.y.Z` | Bug fixes, performance improvements, doc fixes -- no new features, nothing breaks | `v0.2.1`, `v0.2.2` |
| **Alpha** | `x.y.z-alpha` / `-alpha.N` | Earliest testing -- feature incomplete, actively changing, expect breakage; for maintainers/close contributors only | `v0.3.0-alpha`, `v0.3.0-alpha.2` |
| **Beta** | `x.y.z-beta` / `-beta.N` | Feature-complete but unstable -- hunting for bugs; safe for wider early testers | `v0.3.0-beta`, `v0.3.0-beta.2` |
| **RC** | `x.y.z-rc.N` | "This is what we intend to ship" -- no known bugs, waiting to see if anything surfaces | `v0.3.0-rc.1` |

**Ordering:** a pre-release always sorts before the version it leads to: `0.3.0-alpha < 0.3.0-alpha.1 < 0.3.0-beta < 0.3.0-rc.1 < 0.3.0`. `v0.3.0-rc.1` means "release candidate for the upcoming 0.3.0," not "something after 0.3.0."

## Cutting a release

```bash
git tag v0.3.0-alpha.1   # start testing a new feature
git push origin v0.3.0-alpha.1

# ...iterate...
git tag v0.3.0-beta.1
git push origin v0.3.0-beta.1

# ...stabilizes...
git tag v0.3.0-rc.1
git push origin v0.3.0-rc.1

# ...no issues found...
git tag v0.3.0
git push origin v0.3.0
```

`scripts/bump-version.sh` updates the version in `[workspace.package]` and all `rl-*` entries under
`[workspace.dependencies]` in one step:

```bash
scripts/bump-version.sh 1.3.0
scripts/bump-version.sh patch   # or minor / major, computed from the current version
```
