# Multi-Format Archive Extraction Design

## 1. Overview
This design expands `cozip` and its ecosystem (`cozip_desktop`, OS integrations) to support extracting multi-format archives in addition to `.zip` and `.cozip`.

### Supported Formats
- **ZIP**: `.zip`
- **CoZip (PDeflate)**: `.cozip`
- **TAR & Compressed TAR**: `.tar`, `.tar.gz`, `.tgz`, `.tar.bz2`, `.tbz2`, `.tar.xz`, `.txz`
- **RAR**: `.rar`
- **7-Zip**: `.7z`

---

## 2. Architecture & Components

```
+-----------------------------------------------------------------------+
|                             cozip_desktop                             |
|        (GUI Launcher, Drag & Drop, Extract Job Orchestration)        |
+-----------------------------------++----------------------------------+
                                    ||
                                    \/
+-----------------------------------------------------------------------+
|                                cozip                                  |
|                                                                       |
|  +---------------------------+     +-------------------------------+  |
|  | inspect_archive_from_name |     | extract_archive_from_name     |  |
|  +-------------+-------------+     +---------------+---------------+  |
|                |                                   |                  |
|                v                                   v                  |
|       Format Inspection                    Format Dispatcher          |
|    (Magic Bytes + Extension)        (ZIP / CoZip / TAR / 7Z / RAR)    |
+-----------------------------------------------------------------------+
```

---

## 3. Crate Dependencies (`cozip/Cargo.toml`)

The following dependencies will be added to `cozip`:
- `tar`: TAR format reader
- `flate2`: gzip decompression
- `bzip2`: bzip2 decompression
- `xz2` or `lzma-rs`: xz decompression
- `sevenz-rust`: 7z extraction
- `unrar` / pure-rust RAR decoding: RAR extraction

---

## 4. `cozip` Library API Changes

### 4.1 Format Definition
Expand `CoZipArchiveFormat`:
```rust
pub enum CoZipArchiveFormat {
    Zip,
    PDeflate,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    Rar,
    SevenZip,
}
```

### 4.2 Inspection (`inspect_archive_from_name`)
Detection logic by Magic Bytes and File Extensions:
- `PK\x03\x04` / `PK\x05\x06` -> `Zip`
- `PDS0` / `CZPD` / `CZAR` -> `PDeflate`
- `7z\xBC\xAF\x27\x1C` -> `SevenZip`
- `Rar!\x1A\x07` -> `Rar`
- `\x1F\x8B` -> `TarGz` (if `.tar.gz` or `.tgz` or tar header inside)
- `BZh` -> `TarBz2` (if `.tar.bz2` or `.tbz2` or tar header inside)
- `\xFD7zXZ\x00` -> `TarXz` (if `.tar.xz` or `.txz` or tar header inside)
- `ustar` at offset 257 or `.tar` extension -> `Tar`

### 4.3 Extraction Dispatcher (`extract_archive_from_name`)
A unified extraction function:
```rust
pub fn extract_archive_from_name<PIn: AsRef<Path>, POut: AsRef<Path>>(
    archive_path: PIn,
    output_dir: POut,
) -> Result<CoZipStats, CoZipError>;
```
Each format handler extracts files safely, preventing directory traversal attacks (`..` components in paths) and creating necessary parent directories.

---

## 5. `cozip_desktop` GUI & CLI Updates

1. Update `ArchiveFormat` and `ExtractTask` in `src/cozip_desktop/src/launch.rs` to support the new formats.
2. Update `src/cozip_desktop/src/jobs.rs` to invoke the unified extraction API.
3. Update GUI file dialog filters and drag-and-drop handlers to accept `.rar`, `.7z`, `.tar`, `.tar.gz`, `.tgz`, `.tar.bz2`, `.tbz2`, `.tar.xz`, `.txz`.

---

## 6. OS Context Menu Integrations

1. **Linux Packaging (`packaging/linux`)**:
   - Update `cozip-extract-here-servicemenu.desktop` and mime files to include MIME types:
     `application/x-rar`, `application/x-7z-compressed`, `application/x-tar`, `application/gzip`, `application/x-bzip2`, `application/x-xz`.
   - Update Nautilus/Nemo/Caja filemanager scripts.
2. **Windows Shell (`cozip_win_shell`)**:
   - Ensure context menu handlers accept all supported archive extensions.

---

## 7. Verification Plan
1. Unit tests for each archive format extraction in `cozip`.
2. Workspace integration test `cargo test --workspace`.
3. CLI/Desktop argument parsing & extraction plan verification tests.
