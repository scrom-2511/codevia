use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum FsError {
    #[error("Access denied: '{0}' is outside the workspace.")]
    AccessDenied(String),
    #[error("Path not found: '{0}'")]
    NotFound(String),
    #[error("File already exists: '{0}'")]
    AlreadyExists(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct FileSystem {
    cwd: PathBuf,
    excluded_dirs: HashSet<&'static str>,
}

impl FileSystem {
    pub fn new() -> Result<Self, std::io::Error> {
        let cwd = env::current_dir()?.canonicalize()?;
        let excluded_dirs = [
            ".git",
            ".venv",
            "venv",
            "__pycache__",
            ".pytest_cache",
            ".mypy_cache",
            ".ruff_cache",
            "node_modules",
            "dist",
            "build",
            ".next",
            ".cache",
        ]
        .into_iter()
        .collect();

        Ok(Self { cwd, excluded_dirs })
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    pub fn excluded_dirs(&self) -> &HashSet<&'static str> {
        &self.excluded_dirs
    }

    fn normalize_path(&self, rel_path: &str) -> PathBuf {
        let clean_path = rel_path.trim_start_matches(|c| c == '/' || c == '\\');

        let mut full_path = self.cwd.clone();

        for component in Path::new(clean_path).components() {
            match component {
                Component::Normal(c) => full_path.push(c),
                Component::ParentDir => {
                    full_path.pop();
                }
                _ => {}
            }
        }

        full_path
    }

    pub fn resolve_safe_path(&self, path: &str) -> Result<PathBuf, FsError> {
        let normalized = self.normalize_path(path);
        println!("normalized: {}", normalized.display());

        if !normalized.exists() {
            return Err(FsError::NotFound(path.to_string()));
        }

        let canonical = normalized.canonicalize()?;
        println!("canonical: {}", canonical.display());

        if !canonical.starts_with(&self.cwd) {
            return Err(FsError::AccessDenied(path.to_string()));
        }

        Ok(canonical)
    }

    pub fn resolve_write_path(&self, path: &str) -> Result<PathBuf, FsError> {
        let normalized = self.normalize_path(path);

        if !normalized.starts_with(&self.cwd) {
            return Err(FsError::AccessDenied(path.to_string()));
        }

        let mut ancestor = normalized.as_path();
        while !ancestor.exists() {
            if let Some(parent) = ancestor.parent() {
                ancestor = parent;
            } else {
                break;
            }
        }

        if ancestor.exists() {
            let canonical_ancestor = ancestor.canonicalize()?;
            if !canonical_ancestor.starts_with(&self.cwd) {
                return Err(FsError::AccessDenied(path.to_string()));
            }
        }

        Ok(normalized)
    }

    pub fn create_file(&self, file_path: &str) -> String {
        match self.try_create_file(file_path) {
            Ok(msg) => msg,
            Err(e) => format!("Error creating file: {}", e),
        }
    }

    fn try_create_file(&self, file_path: &str) -> Result<String, FsError> {
        let path = self.resolve_write_path(file_path)?;

        if path.exists() {
            return Err(FsError::AlreadyExists(file_path.to_string()));
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        File::create(&path)?;

        Ok(format!("Successfully created {}", file_path))
    }
}
