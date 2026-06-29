# RustUse Facade Report

## Summary

- Root: `\\?\C:\Users\Joshua\Desktop\RustUse\git_local\use-constants`
- Facade: `use-constants`
- Git: `yes`
- Cargo.toml: `yes`
- crates/: `yes`
- Child crate manifests: `7`
- Status: **warning**

## Contents

- [Action Plan](#action-plan)
- [Facade Shape](#facade-shape)
- [Repository Surface](#repository-surface)
- [Cargo Manifest Health](#cargo-manifest-health)
- [Child Crates](#child-crates)
- [Crate Documentation Consistency](#crate-documentation-consistency)
- [Standard File Consistency](#standard-file-consistency)
- [Non-standard Paths](#non-standard-paths)
- [Tooling Configuration](#tooling-configuration)
- [Development Environment](#development-environment)
- [CI/CD Surface](#cicd-surface)
- [Documentation Surface](#documentation-surface)
- [Release Surface](#release-surface)
- [Generated / Local Artifacts](#generated--local-artifacts)
- [Notes](#notes)

## Action Plan

- Restore missing release surface files.
- Address documentation surface warnings.
- Restore 4 missing required standard file(s).
  - Missing `CONTRIBUTING.md` (Contribution guide)
  - Missing `GOVERNANCE.md` (Governance)
  - Missing `MAINTAINERS.md` (Maintainers)
  - Missing `RELEASE.md` (Release notes)

## Facade Shape

| Check | Status |
|---|---:|
| `.git` | yes |
| `Cargo.toml` | yes |
| `crates/` | yes |
| Child crate manifests | 7 |

## Repository Surface

- Status: **warning**
- Required files: `19/23`
- Optional files: `1/1`
- Required directories: `3/3`
- Optional directories: `4/4`

## Cargo Manifest Health

- Status: **ok**
- Manifests inspected: `8`
- Issues: `0`
- Errors: `0`
- Warnings: `0`
- Invalid crates.io category slugs: `0`

### Manifest Inventory

| Kind | Package | Status | Issues | Manifest |
|---|---|---:|---:|---|
| `workspace-root` | `<none>` | ok | 0 | `Cargo.toml` |
| `child-package` | `use-astronomical-constants` | ok | 0 | `crates/use-astronomical-constants/Cargo.toml` |
| `child-package` | `use-chemical-constants` | ok | 0 | `crates/use-chemical-constants/Cargo.toml` |
| `child-package` | `use-computing-constants` | ok | 0 | `crates/use-computing-constants/Cargo.toml` |
| `facade-package` | `use-constants` | ok | 0 | `crates/use-constants/Cargo.toml` |
| `child-package` | `use-earth-constants` | ok | 0 | `crates/use-earth-constants/Cargo.toml` |
| `child-package` | `use-math-constants` | ok | 0 | `crates/use-math-constants/Cargo.toml` |
| `child-package` | `use-physical-constants` | ok | 0 | `crates/use-physical-constants/Cargo.toml` |

## Child Crates

| Kind | Crate | Manifest | README | lib.rs | prelude.rs | Status | Issues |
|---|---|---|---:|---:|---:|---:|---:|
| `child` | `use-astronomical-constants` | `crates/use-astronomical-constants/Cargo.toml` | yes | yes | no | ok | 0 |
| `child` | `use-chemical-constants` | `crates/use-chemical-constants/Cargo.toml` | yes | yes | no | ok | 0 |
| `child` | `use-computing-constants` | `crates/use-computing-constants/Cargo.toml` | yes | yes | no | ok | 0 |
| `facade` | `use-constants` | `crates/use-constants/Cargo.toml` | yes | yes | yes | ok | 0 |
| `child` | `use-earth-constants` | `crates/use-earth-constants/Cargo.toml` | yes | yes | no | ok | 0 |
| `child` | `use-math-constants` | `crates/use-math-constants/Cargo.toml` | yes | yes | no | ok | 0 |
| `child` | `use-physical-constants` | `crates/use-physical-constants/Cargo.toml` | yes | yes | no | ok | 0 |

## Crate Documentation Consistency

| Status | Kind | Crate | README | lib.rs | prelude.rs | Notes |
|---|---|---|---:|---:|---:|---|
| ok | `child` | `use-astronomical-constants` | yes | yes | no | ok |
| ok | `child` | `use-chemical-constants` | yes | yes | no | ok |
| ok | `child` | `use-computing-constants` | yes | yes | no | ok |
| ok | `facade` | `use-constants` | yes | yes | yes | ok |
| ok | `child` | `use-earth-constants` | yes | yes | no | ok |
| ok | `child` | `use-math-constants` | yes | yes | no | ok |
| ok | `child` | `use-physical-constants` | yes | yes | no | ok |

## Standard File Consistency

| Status | Required | Present | Path | Purpose |
|---|---:|---:|---|---|
| ok | yes | yes | `.clippy.toml` | Clippy configuration |
| ok | yes | yes | `.editorconfig` | EditorConfig |
| ok | yes | yes | `.gitattributes` | Git attributes |
| ok | yes | yes | `.gitignore` | Git ignore rules |
| ok | yes | yes | `.gitleaks.toml` | Gitleaks configuration |
| ok | yes | yes | `.rustfmt.toml` | Rustfmt configuration |
| ok | yes | yes | `.taplo.toml` | Taplo configuration |
| ok | yes | yes | `.trivyignore` | Trivy ignore rules |
| ok | yes | yes | `Cargo.lock` | Cargo lockfile |
| ok | yes | yes | `Cargo.toml` | Workspace manifest |
| ok | yes | yes | `CHANGELOG.md` | Changelog |
| warning | yes | no | `CONTRIBUTING.md` | Contribution guide |
| ok | yes | yes | `deny.toml` | cargo-deny configuration |
| warning | yes | no | `GOVERNANCE.md` | Governance |
| ok | yes | yes | `LICENSE-APACHE` | Apache license |
| ok | yes | yes | `LICENSE-MIT` | MIT license |
| warning | yes | no | `MAINTAINERS.md` | Maintainers |
| ok | yes | yes | `Makefile` | Make targets |
| ok | yes | yes | `README.md` | Facade README |
| ok | yes | yes | `release-plz.toml` | release-plz configuration |
| warning | yes | no | `RELEASE.md` | Release notes |
| ok | yes | yes | `RELEASING.md` | Release process |
| ok | yes | yes | `rust-toolchain.toml` | Rust toolchain pin |
| ok | no | yes | `.gitlab-ci.yml` | GitLab CI |

## Non-standard Paths

- Status: **ok**
- Present: `0/1`

- No non-standard paths detected.

## Tooling Configuration

- Status: **ok**
- Present: `8/8`

| Surface | Present |
|---|---:|
| `.cargo` | yes |
| `.clippy.toml` | yes |
| `.rustfmt.toml` | yes |
| `.taplo.toml` | yes |
| `deny.toml` | yes |
| `.gitleaks.toml` | yes |
| `.trivyignore` | yes |
| `rust-toolchain.toml` | yes |

## Development Environment

- Status: **ok**
- Present: `6/6`

| Surface | Present |
|---|---:|
| `.cargo/config.toml` | yes |
| `.devcontainer/devcontainer.json` | yes |
| `.devcontainer/post-create.sh` | yes |
| `scripts/bootstrap-dev-tools.ps1` | yes |
| `scripts/bootstrap-dev-tools.sh` | yes |
| `scripts/sync-mirrors.sh` | yes |

## CI/CD Surface

- Status: **ok**
- Required GitHub CI/CD surface: `17/17`
- Required GitHub workflows: `14/14`
- GitLab CI surface: `2/2`
- Release CI/CD surface: `1/1`

### GitHub CI/CD Surface

| Required | Surface | Present |
|---:|---|---:|
| yes | `.github/` | yes |
| yes | `.github/workflows/` | yes |
| yes | `.github/dependabot.yml` | yes |

### Required GitHub Workflows

| Workflow | Present |
|---|---:|
| `.github/workflows/advisory-rust-quality.yml` | yes |
| `.github/workflows/cargo-audit.yml` | yes |
| `.github/workflows/cargo-deny.yml` | yes |
| `.github/workflows/ci.yml` | yes |
| `.github/workflows/codeql.yml` | yes |
| `.github/workflows/facade-publish-readiness.yml` | yes |
| `.github/workflows/mirror.yml` | yes |
| `.github/workflows/publish-readiness.yml` | yes |
| `.github/workflows/pull-request.yml` | yes |
| `.github/workflows/release-plz-pr.yml` | yes |
| `.github/workflows/release-plz-release.yml` | yes |
| `.github/workflows/sbom.yml` | yes |
| `.github/workflows/secrets.yml` | yes |
| `.github/workflows/trivy.yml` | yes |

### GitLab CI Surface

| Surface | Present |
|---|---:|
| `.gitlab/` | yes |
| `.gitlab-ci.yml` | yes |

### Release CI/CD Surface

| Surface | Present |
|---|---:|
| `release-plz.toml` | yes |

## Documentation Surface

- Status: **warning**
- Present: `2/5`

| Surface | Present |
|---|---:|
| `README.md` | yes |
| `CHANGELOG.md` | yes |
| `CONTRIBUTING.md` | no |
| `GOVERNANCE.md` | no |
| `MAINTAINERS.md` | no |

## Release Surface

- Status: **warning**
- Present: `4/5`

| Surface | Present |
|---|---:|
| `release-plz.toml` | yes |
| `RELEASE.md` | no |
| `RELEASING.md` | yes |
| `CHANGELOG.md` | yes |
| `Cargo.lock` | yes |

## Generated / Local Artifacts

| Path | Meaning |
|---|---|
| `target` | Cargo build output |
| `target/flycheck0` | rust-analyzer flycheck output |
| `target/flycheck11` | rust-analyzer flycheck output |
| `target/flycheck8` | rust-analyzer flycheck output |
| `target/flycheck9` | rust-analyzer flycheck output |

## Notes

- This report is generated from the local filesystem.
- A facade repository is expected to contain `.git`, `Cargo.toml`, and `crates/`.
- Child crates are direct directories under `crates/` with their own `Cargo.toml`.
- Generated and local artifacts are reported separately from required standard files.
