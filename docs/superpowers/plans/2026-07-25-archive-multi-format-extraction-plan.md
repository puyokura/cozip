# Multi-Format Archive Extraction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable extraction of RAR, TAR.GZ, TGZ, TAR.BZ2, TAR.XZ, TAR, and 7Z archives across `cozip`, `cozip_desktop`, CLI, and OS desktop integrations.

**Architecture:** Integrate Rust extraction crates (`tar`, `flate2`, `bzip2`, `xz2`, `sevenz-rust`, `unrar`) into `cozip`. Update `inspect_archive_from_name` with format detection, implement a unified extraction entry point `extract_archive_from_name`, and integrate into `cozip_desktop` and OS launcher specifications.

**Tech Stack:** Rust 2024, `tar`, `flate2`, `bzip2`, `xz2`, `sevenz-rust`, `unrar`, `cozip`, `cozip_desktop`, GPUI.

## Global Constraints
- Target formats: ZIP, CoZip (PDeflate), RAR, TAR, TAR.GZ (TGZ), TAR.BZ2 (TBZ2), TAR.XZ (TXZ), 7Z.
- Workspace builds and passes all tests: `cargo test --workspace`.
- Safe file path handling: prevent path traversal (Zip Slip) attacks on extracted files.

---

### Task 1: Add dependencies & expand `CoZipArchiveFormat` in `cozip`

**Files:**
- Modify: `src/cozip/Cargo.toml`
- Modify: `src/cozip/src/lib.rs`

**Interfaces:**
- Consumes: Existing `CoZipArchiveFormat` in `cozip`.
- Produces: Expanded `CoZipArchiveFormat` enum with `Tar`, `TarGz`, `TarBz2`, `TarXz`, `Rar`, `SevenZip` variants and `as_str()` method.

- [ ] **Step 1: Write the failing test**
Add a test in `src/cozip/src/lib.rs` checking `CoZipArchiveFormat::as_str` for the new variants (`tar`, `tar.gz`, `tar.bz2`, `tar.xz`, `rar`, `7z`).

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p cozip test_archive_format_variants`
Expected: FAIL due to missing enum variants.

- [ ] **Step 3: Add dependencies and implement variants**
In `src/cozip/Cargo.toml`, add `tar`, `flate2`, `bzip2`, `xz2`, `sevenz-rust`, `unrar`.
In `src/cozip/src/lib.rs`, add `Tar`, `TarGz`, `TarBz2`, `TarXz`, `Rar`, `SevenZip` variants to `CoZipArchiveFormat` and implement `as_str()`.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p cozip test_archive_format_variants`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add src/cozip/Cargo.toml src/cozip/src/lib.rs
git commit -m "feat(cozip): add archive format variants and extraction dependencies"
```

---

### Task 2: Implement format detection in `inspect_archive_from_name`

**Files:**
- Modify: `src/cozip/src/lib.rs`

**Interfaces:**
- Consumes: File path.
- Produces: `inspect_archive_from_name(path) -> Result<CoZipArchiveInfo, CoZipError>`.

- [ ] **Step 1: Write the failing test**
Add tests in `src/cozip/src/lib.rs` that create dummy/mock files with magic header bytes or extensions for `.tar.gz`, `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar`, `.rar`, `.7z` and verify `inspect_archive_from_name` returns correct `CoZipArchiveInfo`.

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p cozip test_inspect_archive_multi_format`
Expected: FAIL ("unsupported archive signature")

- [ ] **Step 3: Implement magic byte + extension inspection**
Update `inspect_archive_from_name` in `src/cozip/src/lib.rs` to detect:
- `\x1F\x8B` -> TarGz
- `BZh` -> TarBz2
- `\xFD7zXZ\x00` -> TarXz
- `7z\xBC\xAF\x27\x1C` -> SevenZip
- `Rar!\x1A\x07` -> Rar
- `ustar` / `.tar` extension -> Tar

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p cozip test_inspect_archive_multi_format`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add src/cozip/src/lib.rs
git commit -m "feat(cozip): implement multi-format archive inspection"
```

---

### Task 3: Implement multi-format extraction engine in `cozip`

**Files:**
- Modify: `src/cozip/src/lib.rs`

**Interfaces:**
- Consumes: `archive_path`, `output_dir`.
- Produces: `extract_archive_from_name(archive_path, output_dir) -> Result<CoZipStats, CoZipError>`.

- [ ] **Step 1: Write the failing test**
Create test functions in `src/cozip/src/lib.rs` that build valid sample archives (`.tar.gz`, `.tar.bz2`, `.tar.xz`, `.7z`) in memory/tempdir and test `extract_archive_from_name`.

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p cozip test_extract_multi_format`
Expected: FAIL

- [ ] **Step 3: Implement extraction for TAR variants, 7Z, and RAR**
Implement `extract_archive_from_name`:
- Route `Zip` -> existing ZIP extraction logic.
- Route `PDeflate` -> existing PDeflate extraction logic.
- Route `Tar` / `TarGz` / `TarBz2` / `TarXz` -> unpack using `tar::Archive` with decoder wrappers (`flate2::read::GzDecoder`, `bzip2::read::BzDecoder`, `xz2::read::XzDecoder`). Sanitize paths to prevent directory traversal.
- Route `SevenZip` -> unpack using `sevenz_rust::decompress_file`.
- Route `Rar` -> unpack using `unrar` crate or fallback extractor.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p cozip test_extract_multi_format`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add src/cozip/src/lib.rs
git commit -m "feat(cozip): implement multi-format extraction engine"
```

---

### Task 4: Integrate multi-format extraction into `cozip_desktop`

**Files:**
- Modify: `src/cozip_desktop/src/launch.rs`
- Modify: `src/cozip_desktop/src/jobs.rs`

**Interfaces:**
- Consumes: CLI args and `extract` commands.
- Produces: Updated `LaunchRequest` parsing and job execution for all archive formats.

- [ ] **Step 1: Write the failing test**
Add a test in `src/cozip_desktop/src/launch.rs` parsing `extract --here archive.tar.gz`, `archive.7z`, `archive.rar`.

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p cozip_desktop test_launch_extract_multi_format`
Expected: FAIL

- [ ] **Step 3: Update `launch.rs` and `jobs.rs`**
Extend `ArchiveFormat` in `launch.rs` to include `Tar`, `TarGz`, `TarBz2`, `TarXz`, `Rar`, `SevenZip` (or map them into extract task handlers). Update argument parser and job execution.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p cozip_desktop test_launch_extract_multi_format`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add src/cozip_desktop/src/launch.rs src/cozip_desktop/src/jobs.rs
git commit -m "feat(cozip_desktop): support multi-format extraction in desktop app"
```

---

### Task 5: Update OS Desktop Integration files

**Files:**
- Modify: `packaging/linux/cozip-extract-here-servicemenu.desktop`
- Modify: `packaging/linux/cozip-extract-details-servicemenu.desktop`
- Modify: `packaging/linux/install.sh`

**Interfaces:**
- System MIME type associations for KDE Dolphin and Linux file managers.

- [ ] **Step 1: Update desktop entries and installation scripts**
Add `application/x-rar`, `application/x-7z-compressed`, `application/x-tar`, `application/gzip`, `application/x-bzip2`, `application/x-xz` to MimeType lines.

- [ ] **Step 2: Run full workspace test**
Run: `cargo test --workspace`
Expected: ALL PASS

- [ ] **Step 3: Commit**
```bash
git add packaging/linux/
git commit -m "feat(packaging): update Linux servicemenus for multi-format extraction"
```
