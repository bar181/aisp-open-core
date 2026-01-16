# Publishing aisp to crates.io

This guide explains how to publish the `aisp` crate to [crates.io](https://crates.io/).

## Prerequisites

### 1. Create a crates.io Account

1. Go to [https://crates.io/](https://crates.io/)
2. Click **"Log in with GitHub"** in the top right
3. Authorize crates.io to access your GitHub account (@bar181)
4. Your account will be created automatically

### 2. Generate an API Token

1. Go to [https://crates.io/settings/tokens](https://crates.io/settings/tokens)
2. Click **"New Token"**
3. Enter a name (e.g., "aisp-publishing")
4. Select scopes:
   - `publish-new` - Create new crates
   - `publish-update` - Update existing crates
5. Click **"Create"**
6. **Copy the token** (shown only once!)

### 3. Login via Cargo

```bash
cargo login <your-api-token>
```

This saves the token to `~/.cargo/credentials.toml`.

## Pre-Publishing Checklist

### Verify Cargo.toml

```toml
[package]
name = "aisp"
version = "0.1.0"  # Increment for each release
edition = "2021"
authors = ["Bradley Ross (https://github.com/bar181)"]
description = "AISP 5.1 document validation library - AI Symbolic Protocol with <2% ambiguity"
license = "MIT OR Apache-2.0"
repository = "https://github.com/bar181/aisp-open-core"
homepage = "https://github.com/bar181/aisp-open-core"
documentation = "https://docs.rs/aisp"
readme = "README.md"
keywords = ["aisp", "ai", "validation", "specification", "type-theory"]
categories = ["development-tools", "parser-implementations"]
```

### Create README.md

```bash
# Ensure README.md exists in aisp-rust/
cat aisp-rust/README.md
```

### Run All Tests

```bash
cd aisp-rust
cargo test --all-features
```

### Build Documentation

```bash
cargo doc --no-deps --open
```

### Check Package

```bash
# Dry-run to check what will be published
cargo publish --dry-run
```

## Publishing

### First Release

```bash
cd aisp-rust
cargo publish
```

### Subsequent Releases

1. Update version in `Cargo.toml`:
   ```toml
   version = "0.1.1"  # or 0.2.0, 1.0.0, etc.
   ```

2. Commit and tag:
   ```bash
   git add Cargo.toml
   git commit -m "Release aisp v0.1.1"
   git tag v0.1.1
   git push && git push --tags
   ```

3. Publish:
   ```bash
   cargo publish
   ```

## Versioning Guidelines

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

### Pre-1.0.0 Guidelines

- `0.1.x` - Initial development, API may change
- `0.2.0` - Significant feature addition
- `0.x.0` - Breaking changes allowed before 1.0

## Post-Publishing

### Verify Publication

1. Check [https://crates.io/crates/aisp](https://crates.io/crates/aisp)
2. Documentation auto-generates at [https://docs.rs/aisp](https://docs.rs/aisp)

### Yanking (if needed)

```bash
# Remove a broken version (does NOT delete, prevents new installs)
cargo yank --vers 0.1.0

# Undo yank
cargo yank --vers 0.1.0 --undo
```

## Common Issues

### "crate name already exists"
The name `aisp` may be taken. Check availability at crates.io first.

### "missing README"
Ensure `readme = "README.md"` in Cargo.toml and the file exists.

### "license not accepted"
Use SPDX identifiers: `MIT`, `Apache-2.0`, or `MIT OR Apache-2.0`.

### "too many keywords"
Maximum 5 keywords allowed.

### "category not recognized"
Valid categories: https://crates.io/categories

## GitHub Actions CI/CD (Optional)

Add `.github/workflows/publish.yml`:

```yaml
name: Publish to crates.io

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}
        working-directory: aisp-rust
```

Add `CRATES_IO_TOKEN` secret in GitHub repository settings.

## Links

- **crates.io**: https://crates.io/
- **API Token**: https://crates.io/settings/tokens
- **Documentation**: https://doc.rust-lang.org/cargo/reference/publishing.html
- **Categories**: https://crates.io/categories
- **Semver**: https://semver.org/
