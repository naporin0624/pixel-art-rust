---
description: Create a GitHub release with compiled binaries
---

# Release Command

Create a new GitHub release for the mosaic-rust project with compiled binaries and comprehensive release notes.

## Usage
`/project:release <version> [release_notes]`

Examples:
- `/project:release v0.1.0`
- `/project:release v0.2.0 "Added new color matching algorithm"`
- `/project:release v1.0.0 "First stable release with performance improvements"`

## Parameters
1. **version**: Version tag (format: "vX.Y.Z")
2. **(optional) release_notes**: Additional release notes (auto-generated if not provided)

## Task
This command will:

1. **Pre-release checks**:
   - Verify working directory is clean
   - Check that release binaries exist
   - Validate version format
   - Update Cargo.toml version if needed

2. **Build release binaries** (if not current):
   - `cargo build --release` for Linux binary
   - Cross-compile for Windows if tools available
   - Verify binary integrity

3. **Create git tag**:
   - Create annotated tag with version
   - Push tag to remote repository

4. **Create GitHub release**:
   - Generate comprehensive release notes including:
     - New features and improvements
     - Performance optimizations
     - Bug fixes
     - Breaking changes (if any)
     - Installation instructions
   - Upload binary assets:
     - `mosaic-rust` (Linux/macOS binary)
     - `mosaic-rust.exe` (Windows binary)
   - Mark as latest release

5. **Generate release notes template**:
   ```markdown
   ## 🎨 Mosaic Art Generator v{version}
   
   A high-performance Rust implementation for creating stunning mosaic art.
   
   ### ✨ New Features
   - [Auto-generated from commit messages since last release]
   
   ### 🚀 Performance Improvements  
   - [Auto-generated from performance-related commits]
   
   ### 🐛 Bug Fixes
   - [Auto-generated from fix commits]
   
   ### 📦 Installation
   
   #### Download Binaries
   - **Linux/macOS**: Download `mosaic-rust`
   - **Windows**: Download `mosaic-rust.exe`
   
   #### Build from Source
   ```bash
   git clone <repo-url>
   cd mosaic-rust
   cargo build --release
   ```
   
   ### 🔄 What's Changed
   [Full changelog with commit details]
   
   ### 🙏 Acknowledgments
   Special thanks to contributors and the Rust community.
   ```

## Available gh Commands Reference

### Release Management
- `gh release create <tag>`: Create a new release
- `gh release upload <tag> <files>`: Upload files to existing release  
- `gh release edit <tag>`: Edit an existing release
- `gh release list`: List all releases
- `gh release view <tag>`: View release details

### Tag Management
- `git tag -a <tag> -m "message"`: Create annotated tag
- `git push origin <tag>`: Push tag to remote
- `git tag -l`: List existing tags

### Binary Upload Options
- Support for multiple file uploads
- Automatic file type detection
- Compression options for large binaries

## Prerequisites
- Clean git working directory
- Compiled release binaries in `target/release/`
- GitHub CLI (gh) authenticated
- Proper repository permissions

## Error Handling
- Validate binaries exist before release
- Check for duplicate version tags
- Verify GitHub authentication
- Rollback on partial failures

The command will provide detailed output showing each step and final release URL for easy access.