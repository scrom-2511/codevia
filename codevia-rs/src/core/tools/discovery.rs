use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;
use std::time::UNIX_EPOCH;

use crate::core::tools::WorkspaceErrors;
use crate::core::tools::file_system::FileSystem;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub size_bytes: u64,
    pub created_time: Option<f64>,
    pub modified_time: Option<f64>,
    pub is_file: bool,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
pub struct FilesAndDirectories {
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

pub struct Discovery<'a> {
    fs: &'a FileSystem,
}

impl<'a> Discovery<'a> {
    pub fn new(fs: &'a FileSystem) -> Self {
        Self { fs }
    }

    pub fn list_files(&self, path: &str, recursive: bool) -> Result<Vec<String>, WorkspaceErrors> {
        let directory = self.fs.resolve_safe_path(path)?;
        let mut result = Vec::new();

        if !recursive {
            for entry in fs::read_dir(&directory)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    if let Ok(rel) = entry.path().strip_prefix(&directory) {
                        result.push(rel.display().to_string());
                    }
                }
            }
            return Ok(result);
        }

        self.collect_entries(&directory, &directory, true, false, &mut result)?;
        Ok(result)
    }

    pub fn list_directories(
        &self,
        path: &str,
        recursive: bool,
    ) -> Result<Vec<String>, WorkspaceErrors> {
        let directory = self.fs.resolve_safe_path(path)?;
        let mut result = Vec::new();

        if !recursive {
            for entry in fs::read_dir(&directory)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                let file_name = entry.file_name();
                let name_str = file_name.display();

                if file_type.is_dir()
                    && !self
                        .fs
                        .excluded_dirs()
                        .contains(&name_str.to_string().as_ref())
                {
                    if let Ok(rel) = entry.path().strip_prefix(&directory) {
                        result.push(rel.display().to_string());
                    }
                }
            }
            return Ok(result);
        }

        self.collect_entries(&directory, &directory, false, true, &mut result)?;
        Ok(result)
    }

    fn collect_entries(
        &self,
        current_dir: &Path,
        base_dir: &Path,
        include_files: bool,
        include_dirs: bool,
        results: &mut Vec<String>,
    ) -> Result<(), WorkspaceErrors> {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            let file_name = entry.file_name();
            let name_str = file_name.display();

            if file_type.is_dir() {
                if self
                    .fs
                    .excluded_dirs()
                    .contains(name_str.to_string().as_str())
                {
                    continue;
                }

                if include_dirs {
                    if let Ok(rel) = path.strip_prefix(base_dir) {
                        results.push(rel.display().to_string());
                    }
                }

                self.collect_entries(&path, base_dir, include_files, include_dirs, results)?;
            } else if file_type.is_file() && include_files {
                if let Ok(rel) = path.strip_prefix(base_dir) {
                    results.push(rel.display().to_string());
                }
            }
        }

        Ok(())
    }

    pub fn list_files_and_directories(
        &self,
        path: &str,
        recursive: bool,
    ) -> Result<FilesAndDirectories, WorkspaceErrors> {
        let files = self.list_files(path, recursive)?;
        let directories = self.list_directories(path, recursive)?;

        Ok(FilesAndDirectories { files, directories })
    }

    pub fn find_files(&self, pattern: &str, path: &str) -> Result<String, WorkspaceErrors> {
        if !self.ensure_rg() {
            return Ok("ripgrep is not installed.".to_string());
        }

        let path_obj = self.fs.resolve_safe_path(path)?;

        let mut cmd = Command::new("rg");
        cmd.arg("--files").arg("-g").arg(pattern);

        for d in self.fs.excluded_dirs() {
            cmd.arg("-g").arg(format!("!{}", d));
        }
        cmd.arg(&path_obj);

        let output = cmd.output()?;

        match output.status.code() {
            Some(0) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
            Some(1) => Ok("No files found matching the pattern.".to_string()),
            _ => Ok(format!(
                "ripgrep error:\n{}",
                String::from_utf8_lossy(&output.stderr)
            )),
        }
    }

    pub fn get_file_info(&self, file_path: &str) -> Result<FileInfo, WorkspaceErrors> {
        let path_obj = self.fs.resolve_safe_path(file_path)?;
        let metadata = fs::metadata(&path_obj)?;

        let created_time = metadata
            .created()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs_f64());

        let modified_time = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs_f64());

        let name = path_obj
            .file_name()
            .map(|n| n.display().to_string())
            .unwrap_or_default();

        Ok(FileInfo {
            name,
            size_bytes: metadata.len(),
            created_time,
            modified_time,
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
        })
    }

    pub fn ensure_rg(&self) -> bool {
        if which_rg() {
            return true;
        }

        let os = env::consts::OS;
        let command: Vec<&str> = match os {
            "linux" => vec!["sudo", "apt", "install", "-y", "ripgrep"],
            "macos" => vec!["brew", "install", "ripgrep"],
            "windows" => vec!["winget", "install", "--id", "BurntSushi.ripgrep.MSVC"],
            _ => return false,
        };

        println!("ripgrep is required for file search.");
        println!("Install command: {}", command.join(" "));

        print!("Install it? [y/N] ");
        let _ = io::stdout().flush();

        let mut answer = String::new();
        let stdin = io::stdin();
        if stdin.lock().read_line(&mut answer).is_err() {
            return false;
        }

        if !answer.trim().eq_ignore_ascii_case("y") {
            return false;
        }

        let mut install_cmd = Command::new(command[0]);
        if command.len() > 1 {
            install_cmd.args(&command[1..]);
        }

        match install_cmd.status() {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }

    pub fn search_in_files(
        &self,
        pattern: &str,
        path: &str,
        before_context: usize,
        after_context: usize,
        multiline: bool,
    ) -> Result<String, WorkspaceErrors> {
        if !self.ensure_rg() {
            return Ok("ripgrep is not installed.".to_string());
        }

        let safe_path = self.fs.resolve_safe_path(path)?;

        let mut cmd = Command::new("rg");
        cmd.arg("-n");

        for d in self.fs.excluded_dirs() {
            cmd.arg("-g").arg(format!("!{}", d));
        }

        if before_context > 0 {
            cmd.arg("-B").arg(before_context.to_string());
        }

        if after_context > 0 {
            cmd.arg("-A").arg(after_context.to_string());
        }

        if multiline {
            cmd.arg("-U");
        }

        cmd.arg("--").arg(pattern).arg(&safe_path);

        let output = cmd.output()?;

        match output.status.code() {
            Some(0) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
            Some(1) => Ok("No matches found.".to_string()),
            _ => Ok(format!(
                "ripgrep error:\n{}",
                String::from_utf8_lossy(&output.stderr)
            )),
        }
    }

    pub fn search_multiple_patterns(
        &self,
        patterns: &[String],
        path: &str,
        before_context: usize,
        after_context: usize,
        multiline: bool,
    ) -> Result<HashMap<String, String>, WorkspaceErrors> {
        let mut results = HashMap::with_capacity(patterns.len());

        for pattern in patterns {
            let res =
                self.search_in_files(pattern, path, before_context, after_context, multiline)?;
            results.insert(pattern.clone(), res);
        }

        Ok(results)
    }
}

fn which_rg() -> bool {
    let check_cmd = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };

    Command::new(check_cmd)
        .arg("rg")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}
