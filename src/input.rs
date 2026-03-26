use anyhow::{bail, Context, Result};
use std::fmt;
use std::path::{Path, PathBuf};

/// Detected bundle format.
#[derive(Debug)]
pub enum BundleFormat {
    Directory,
    TarGz,
    Zip,
}

impl fmt::Display for BundleFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BundleFormat::Directory => write!(f, "Directory"),
            BundleFormat::TarGz => write!(f, "tar.gz archive"),
            BundleFormat::Zip => write!(f, "ZIP archive"),
        }
    }
}

/// Detect the format of the input bundle.
pub fn detect_format(path: &Path) -> Result<BundleFormat> {
    if path.is_dir() {
        return Ok(BundleFormat::Directory);
    }

    if !path.exists() {
        bail!("Path does not exist: {}", path.display());
    }

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    if name.ends_with(".tgz") || name.ends_with(".tar.gz") {
        Ok(BundleFormat::TarGz)
    } else if name.ends_with(".zip") {
        Ok(BundleFormat::Zip)
    } else {
        bail!(
            "Unrecognized bundle format: {}. Expected .tgz, .tar.gz, .zip, or a directory.",
            path.display()
        );
    }
}

/// Extract a bundle to a temporary directory, or return the path if already a directory.
pub fn extract_bundle(path: &Path) -> Result<(PathBuf, Option<tempdir::TempDir>)> {
    match detect_format(path)? {
        BundleFormat::Directory => Ok((path.to_path_buf(), None)),
        BundleFormat::TarGz => extract_tar_gz(path),
        BundleFormat::Zip => extract_zip(path),
    }
}

fn extract_tar_gz(path: &Path) -> Result<(PathBuf, Option<tempdir::TempDir>)> {
    let tmp = tempdir::TempDir::new("amadiag")?;
    let file =
        std::fs::File::open(path).with_context(|| format!("Failed to open {}", path.display()))?;
    let gz = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(gz);
    // SAFETY: tar::Archive::unpack() (since v0.4.16) rejects absolute paths and
    // path components containing `..`, providing built-in protection against
    // path-traversal (Zip Slip) attacks. The ZIP extraction path uses
    // `enclosed_name()` for the same purpose.
    archive
        .unpack(tmp.path())
        .with_context(|| format!("Failed to extract tar.gz: {}", path.display()))?;

    tracing::info!("Extracted tar.gz bundle to {}", tmp.path().display());
    Ok((tmp.path().to_path_buf(), Some(tmp)))
}

fn extract_zip(path: &Path) -> Result<(PathBuf, Option<tempdir::TempDir>)> {
    let tmp = tempdir::TempDir::new("amadiag")?;
    let file =
        std::fs::File::open(path).with_context(|| format!("Failed to open {}", path.display()))?;
    let mut archive =
        zip::ZipArchive::new(file).with_context(|| format!("Invalid ZIP: {}", path.display()))?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let out_path = tmp.path().join(
            entry
                .enclosed_name()
                .context("Invalid entry name in ZIP archive")?,
        );

        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out_file = std::fs::File::create(&out_path)?;
            std::io::copy(&mut entry, &mut out_file)?;
        }
    }

    tracing::info!("Extracted ZIP bundle to {}", tmp.path().display());
    Ok((tmp.path().to_path_buf(), Some(tmp)))
}

/// Validate a bundle and return a human-readable summary.
pub fn validate_bundle(path: &Path) -> Result<String> {
    let format = detect_format(path)?;
    let (dir, _tmp) = extract_bundle(path)?;

    let mut file_count = 0_usize;
    let mut total_size = 0_u64;
    let mut xml_count = 0_usize;
    let mut log_count = 0_usize;
    let mut csv_count = 0_usize;

    for entry in walkdir::WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        file_count += 1;
        total_size += entry.metadata().map(|m| m.len()).unwrap_or(0);

        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name.ends_with(".xml") {
            xml_count += 1;
        } else if name.ends_with(".log") || name.ends_with(".txt") {
            log_count += 1;
        } else if name.ends_with(".csv") {
            csv_count += 1;
        }
    }

    Ok(format!(
        "Bundle Validation\n\
         ─────────────────\n\
         Path:       {}\n\
         Format:     {format}\n\
         Files:      {file_count}\n\
         Total size: {}\n\
         XML files:  {xml_count}\n\
         Log files:  {log_count}\n\
         CSV files:  {csv_count}\n\
         Status:     {}",
        path.display(),
        format_size(total_size),
        if file_count > 0 {
            "OK"
        } else {
            "EMPTY — no files found in bundle"
        }
    ))
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

// Minimal tempdir implementation (avoids adding another crate)
mod tempdir {
    use std::path::{Path, PathBuf};

    pub struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        pub fn new(prefix: &str) -> std::io::Result<Self> {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "{prefix}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            );
            path.push(unique);
            std::fs::create_dir_all(&path)?;
            Ok(Self { path })
        }

        pub fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn detect_directory_format() {
        let dir = std::env::temp_dir();
        let fmt = detect_format(&dir).unwrap();
        assert!(matches!(fmt, BundleFormat::Directory));
    }

    #[test]
    fn detect_tgz_by_extension() {
        let fmt = detect_format(Path::new("bundle.tgz"));
        // File doesn't exist, but format detection for dirs happens first;
        // for non-existent files we get an error about path not existing.
        assert!(fmt.is_err());
    }

    #[test]
    fn detect_tar_gz_extension() {
        // Create a temp file with .tar.gz extension
        let tmp = tempdir::TempDir::new("amadiag-test").unwrap();
        let p = tmp.path().join("bundle.tar.gz");
        std::fs::write(&p, b"not-a-real-archive").unwrap();
        let fmt = detect_format(&p).unwrap();
        assert!(matches!(fmt, BundleFormat::TarGz));
    }

    #[test]
    fn detect_zip_extension() {
        let tmp = tempdir::TempDir::new("amadiag-test").unwrap();
        let p = tmp.path().join("bundle.zip");
        std::fs::write(&p, b"not-a-real-archive").unwrap();
        let fmt = detect_format(&p).unwrap();
        assert!(matches!(fmt, BundleFormat::Zip));
    }

    #[test]
    fn reject_unknown_extension() {
        let tmp = tempdir::TempDir::new("amadiag-test").unwrap();
        let p = tmp.path().join("bundle.txt");
        std::fs::write(&p, b"hello").unwrap();
        assert!(detect_format(&p).is_err());
    }

    #[test]
    fn reject_nonexistent_path() {
        assert!(detect_format(Path::new("/no/such/path/bundle.zip")).is_err());
    }

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(500), "500 B");
    }

    #[test]
    fn format_size_kb() {
        assert_eq!(format_size(2048), "2.0 KB");
    }

    #[test]
    fn format_size_mb() {
        assert_eq!(format_size(5 * 1024 * 1024), "5.0 MB");
    }

    #[test]
    fn bundle_format_display() {
        assert_eq!(BundleFormat::Directory.to_string(), "Directory");
        assert_eq!(BundleFormat::TarGz.to_string(), "tar.gz archive");
        assert_eq!(BundleFormat::Zip.to_string(), "ZIP archive");
    }

    #[test]
    fn extract_directory_returns_same_path() {
        let dir = std::env::temp_dir();
        let (path, tmp) = extract_bundle(&dir).unwrap();
        assert_eq!(path, dir);
        assert!(tmp.is_none());
    }
}
