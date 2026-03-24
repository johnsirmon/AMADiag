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
    archive
        .unpack(tmp.path())
        .with_context(|| format!("Failed to extract tar.gz: {}", path.display()))?;

    tracing::info!(
        "Extracted tar.gz bundle to {}",
        tmp.path().display()
    );
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
        if file_count > 0 { "OK" } else { "EMPTY — no files found in bundle" }
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
