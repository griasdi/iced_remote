# Releasing

Checklist for a release of the `iced_remote` crate and the
`iced-design` skill plugin. Both carry the same version.

## 1. Pick the version

Under Cargo's 0.x semver rules a public-API break is a minor bump
(`0.1.x` -> `0.2.0`), anything else a patch bump. A protocol change that breaks
existing `ctl` scripts (renamed command, changed reply shape) counts as a
break. Check the Rust API against the last tag:

```sh
cargo semver-checks --baseline-rev "$(git tag --list 'v*' --sort=-v:refname | head -n1)"
```

## 2. Bump the version

- `Cargo.toml`: `version = "X.Y.Z"`, then `cargo update -p iced_remote`.
- `plugin/.claude-plugin/plugin.json`: `"version"`.
- `plugin/package.json`: `"version"`.
- `.claude-plugin/marketplace.json`: the plugin entry's `"version"`.
- `plugin/skills/iced-design/SKILL.md`: frontmatter `version`.

`cargo test --test version` checks that the five agree.

## 3. Update the CHANGELOG

- Rename `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD` and add a fresh empty
  `## [Unreleased]` above it.
- Add `[X.Y.Z]: https://github.com/griasdi/iced_remote/releases/tag/vX.Y.Z`
  at the bottom.
- Point the `[Unreleased]:` link at
  `https://github.com/griasdi/iced_remote/compare/vX.Y.Z...HEAD`.

`scripts/release-notes.sh X.Y.Z` prints the release notes and fails as the
release workflow would.

## 4. Run every gate

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
scripts/release-notes.sh X.Y.Z
cargo build --example demo --bin iced-remote
scripts/smoke.sh
cargo check --target x86_64-pc-windows-gnu --all-targets
cargo deny --log-level error check
cargo package --locked
cargo package --list    # no .claude-plugin, .github, scripts, assets or plugin/package.json entries
```

## 5. Commit, tag, push

```sh
git commit -am "chore(release): X.Y.Z"
git tag -a vX.Y.Z -m "Release X.Y.Z"
git push origin main vX.Y.Z
```

Pushing the tag starts `.github/workflows/release.yml`.

## 6. Watch the release

The release workflow checks the tag against `Cargo.toml` and the CHANGELOG
section, runs the whole CI, publishes to crates.io with Trusted Publishing and
creates the GitHub release titled `X.Y.Z` with the CHANGELOG section as notes.

```sh
gh run watch "$(gh run list --workflow release.yml --limit 1 --json databaseId -q '.[0].databaseId')"
```

- Failure before `publish`: fix on `main`, then move the tag:
  `git tag -d vX.Y.Z && git push origin :refs/tags/vX.Y.Z`, tag and push again.
- Failure in `github-release` only: `gh run rerun <run-id> --failed`. Never
  re-run `publish`: crates.io rejects a second upload of the same version.

## 7. Install locally

```sh
cargo install --path . --bin iced-remote --locked
omp plugin marketplace update iced-remote    # re-copies the skill files
omp plugin install --force iced-design@iced-remote
```

## Trusted Publishing

Already configured on crates.io, nothing to do: `iced_remote` → Settings →
Trusted Publishing has a GitHub publisher for repository
`griasdi/iced_remote`, workflow `release.yml`, environment `release`.
Renaming the workflow file or the environment breaks publishing until this
entry is changed too.
