# 🚀 CLI Template

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
![Development: Active](https://img.shields.io/badge/Development-Active-blue)

A minimal, production-ready Rust template designed to jumpstart command-line interface (CLI) development. It establishes a workspace-friendly structure and comes pre-configured with automated release pipelines, versioning, and cross-platform distribution.

## 🚀 How to Use

### Developing

1. Add a new package: `cargo new <package-name>`
2. Run the package: `cargo run -p <package-name>`

### CI Configuration

The project includes a GitHub Actions workflow for continuous integration. To modify the platforms used for testing (Clippy, tests, and documentation), edit `.github/workflows/ci.yml` and update the `supported_os` list:

```yaml
matrix:
  os: &supported_os [ubuntu-latest, macos-latest, windows-latest]
```

### Publishing and Distributing

This project utilizes a coordinated release pipeline to automate package maintenance and artifact distribution. Together, `release-plz` and `cargo-dist` manage changelogs, semantic versioning, crates.io publication, and the automatic compilation of multi-platform binaries for GitHub Releases.

#### Release-plz

We use `release-plz` to automate updating [CHANGELOG.md](CHANGELOG.md), bumping versions, tagging Git commits, and publishing to `crates.io`.

Steps to enable `release-plz`:

1. **Allow PR Creation**: Navigate to `https://github.com/<user>/<repo>/settings/actions`. Under the **Workflow permissions** section, check the box for **"Allow GitHub Actions to create and approve pull requests"**.
2. **Configure PAT (Required for `cargo-dist` integration)**: Navigate to `https://github.com/<user>/<repo>/settings/secrets/actions` and add a repository secret named `RELEASE_PLZ_TOKEN`.
    * This must be a GitHub Personal Access Token (PAT).
    * If using a **Fine-grained PAT**, grant **`Contents: Read and write`** (to push Git tags) and **`Pull Requests: Read and write`** (to open release PRs) permissions.
    * If using a **Classic PAT**, select the **`repo`** scope.
    *(Note: Using this custom token instead of the default `GITHUB_TOKEN` is required to ensure that Git tags pushed by `release-plz` trigger the `cargo-dist` build workflow).*
3. **Enable `release-plz` for Your Repository**: Update `if: github.repository == 'Atliac/cli-template'` to `if: github.repository == '<your-username>/<your-repo>'` in `.github/workflows/release-plz.yml`.
4. **(Optional) Publishing to `crates.io`**:
    1. **First Publish**: Run `cargo publish` manually from your local machine once. (Crates.io does not allow publishing a brand-new crate via automation).
    2. **Trusted Publishing**: Follow the [crates.io Trusted Publishing guide](https://crates.io/docs/trusted-publishing) to link your GitHub repository.
    3. **Enable in Config**: Edit [release-plz.toml](release-plz.toml) and remove the `publish = false` lines (or set them to `true`).

#### Distributing

We use `cargo-dist` to compile cross-platform binaries, build installer scripts, and automatically attach them to GitHub Releases whenever `release-plz` pushes a Git tag.

1. Install `cargo-dist` locally: `cargo install cargo-dist`
2. Initialize it in your repository: `dist init`
   *(This will guide you through setting up target platforms and generate a `.github/workflows/release.yml` file that triggers on tags created by `release-plz`)*.

Run `dist build` to test the build on your local platform.

You can rerun `dist init` at any time to update your configuration.

## 📜 License

This project is dual-licensed under:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT](https://opensource.org/licenses/MIT)

You may choose either license at your discretion.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
