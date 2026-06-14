# 🚀 CLI Template

![License](https://img.shields.io/crates/l/wgc)
![Development: Active](https://img.shields.io/badge/Development-Active-blue)

A minimal, production-ready Rust template designed to kickstart command-line interface (CLI) development. It establishes a workspace-friendly structure and comes pre-configured with automated release pipelines, versioning, and cross-platform distribution.

## 🚀 How to Use

### Developing

1. Add new package `cargo add <package-name>`
2. Run the package `cargo run -p <package-name>`

### Publishing and Distributing

This project utilizes a coordinated release pipeline to automate package maintenance and artifact distribution. Together, release-plz and cargo-dist manage changelogs, semantic versioning, crates.io publication, and the automatic compilation of multi-platform binaries for GitHub Releases.

#### Release-plz

We use `Release-plz` to automate [CHANGELOG.md](CHANGELOG.md) updating, version bumping, Git tagging, and `crates.io` publishing.

Steps to enable `Release-plz`:

1. **Allow PR Creation**: Navigate to `https://github.com/<user>/<repo>/settings/actions`. Under the **Workflow permissions** section, check the box for **"Allow GitHub Actions to create and approve pull requests"**.
2. **Configure PAT (Required for `cargo-dist` integration)**: Navigate to `https://github.com/<user>/<repo>/settings/secrets/actions` and add a Repository Secret named `RELEASE_PLZ_TOKEN`.
    * This must be a GitHub **Personal Access Token (PAT)**.
    * If using a **Fine-grained PAT**, grant **`Contents: Read and write`** (to push git tags) and **`Pull Requests: Read and write`** (to open release PRs) permissions.
    * If using a **Classic PAT**, select the **`repo`** scope.
    *(Note: Using this custom token instead of the default `GITHUB_TOKEN` is required so that the Git tags pushed by `release-plz` can trigger the `cargo-dist` build workflow).*
3. **Repository Filter**: In `.github/workflows/release-plz.yml`, find the lines containing `if: github.repository == 'Atliac/cli-template'` (present on both jobs) and change them to your own repository path: `if: github.repository == '<your-username>/<your-repo>'`.
4. **(Optional) `crates.io` Publishing**:
    1. **First Publish**: Run `cargo publish` manually from your local machine once. (Crates.io does not allow publishing a brand-new crate via automation).
    2. **Trusted Publishing**: Follow the [crates.io Trusted Publishing guide](https://crates.io/docs/trusted-publishing) to link your GitHub repository.
    3. **Enable in Config**: Edit [release-plz.toml](release-plz.toml) and remove the `publish = false` lines (or set them to `true`).

#### Distributing

We use `cargo-dist` to compile cross-platform binaries, build installer scripts, and attach them to GitHub Releases automatically whenever `release-plz` pushes a tag.

1. Install cargo-dist locally: `cargo install cargo-dist`
2. Initialize it in your repository: `dist init`
   *(This will guide you through setting up target platforms and automatically generate a `.github/workflows/release.yml` file which responds to tags created by `release-plz`)*.

Run `dist build` to build for the current platform. (For testing)

Rerun `dist init` as you wish to change `dist` configurations.

## 📜 License

This project is dual-licensed under:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT](https://opensource.org/licenses/MIT)

You may choose either license at your discretion.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
