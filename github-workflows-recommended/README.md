# GitHub Actions CI/CD Configuration for RVOIP

This directory contains GitHub Actions workflows for continuous integration and deployment.

## Workflows

### 1. `ci.yml` - Continuous Integration

Runs on every push and pull request to ensure code quality.

**What it does**:
- Builds all workspace members
- Runs all tests (unit + integration)
- Checks code formatting (rustfmt)
- Runs clippy lints
- Generates code coverage report

**Triggers**:
- Push to main branch
- All pull requests
- Manual workflow dispatch

### 2. `release.yml` - Release Automation

Automates the release process when a new tag is pushed.

**What it does**:
- Builds release binaries
- Runs full test suite
- Publishes to crates.io
- Creates GitHub release with artifacts
- Updates documentation

**Triggers**:
- Push of version tags (v*.*.*)

### 3. `docs.yml` - Documentation

Builds and deploys API documentation.

**What it does**:
- Generates rustdoc for all crates
- Deploys to GitHub Pages
- Checks doc examples compile

**Triggers**:
- Push to main
- Weekly schedule

## Setup Instructions

1. **Copy workflows to .github/workflows/**
   ```bash
   cp github-workflows-recommended/*.yml .github/workflows/
   ```

2. **Configure secrets in GitHub**:
   - `CRATES_IO_TOKEN` - For publishing to crates.io
   - GitHub token is provided automatically

3. **Enable GitHub Pages** (for docs):
   - Go to repository Settings → Pages
   - Set source to "GitHub Actions"

4. **Customize as needed**:
   - Adjust Rust version in workflows
   - Modify test commands if you have special requirements
   - Add additional checks

## Recommended Workflow Files

See the example files in this directory:
- `ci.yml` - Basic CI pipeline
- `release.yml` - Release automation
- `docs.yml` - Documentation deployment

## Additional Recommendations

### Branch Protection
Configure these in GitHub Settings → Branches:
- Require status checks to pass (CI workflow)
- Require code review before merge
- Require branches to be up to date

### Status Badges
Add to README.md:
```markdown
[![CI](https://github.com/eisenzopf/rvoip/workflows/CI/badge.svg)](https://github.com/eisenzopf/rvoip/actions)
[![docs](https://docs.rs/rvoip/badge.svg)](https://docs.rs/rvoip)
```

### Cargo Configuration
Create `.cargo/config.toml`:
```toml
[build]
# Faster builds during development
incremental = true

[target.x86_64-unknown-linux-gnu]
# Use mold linker for faster linking (if available)
# linker = "clang"
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

## Testing Locally

Before pushing, you can test the workflows locally using `act`:

```bash
# Install act (GitHub Actions local runner)
brew install act  # macOS
# or download from https://github.com/nektos/act

# Run CI workflow
act push

# Run specific job
act -j test
```

## Performance Tips

1. **Cache Cargo dependencies**:
   - Already included in example workflows
   - Reduces build time by 2-3x

2. **Use sparse registry**:
   ```yaml
   env:
     CARGO_REGISTRIES_CRATES_IO_PROTOCOL: sparse
   ```

3. **Parallel testing**:
   ```yaml
   - name: Run tests
     run: cargo test --workspace --all-features -- --test-threads=4
   ```

4. **Matrix builds** (for multi-platform):
   ```yaml
   strategy:
     matrix:
       os: [ubuntu-latest, macos-latest, windows-latest]
       rust: [stable, beta]
   ```

## Questions?

- See GitHub Actions docs: https://docs.github.com/actions
- RVOIP CI issues: Open GitHub issue with `ci` label
