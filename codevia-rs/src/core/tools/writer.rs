use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::core::ErrorKind;
use crate::core::tools::WorkspaceErrors;
use crate::core::tools::file_system::FileSystem;

pub struct Writer<'a> {
    fs: &'a FileSystem,
}

impl<'a> Writer<'a> {
    pub fn new(fs: &'a FileSystem) -> Self {
        Self { fs }
    }

    pub fn write_file(&self, file_path: &str, content: &str) -> Result<String, WorkspaceErrors> {
        let path = self.fs.resolve_write_path(file_path)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content)?;

        Ok(format!("Successfully wrote to {}", file_path))
    }

    pub fn write_multiple_files(
        &self,
        files: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, WorkspaceErrors> {
        let mut results = HashMap::with_capacity(files.len());

        for (file_path, content) in files {
            let result = match self.write_file(file_path, content) {
                Ok(message) => message,
                Err(error) => format!("Error writing file: {}", error),
            };

            results.insert(file_path.clone(), result);
        }

        Ok(results)
    }

    pub fn append_to_file(
        &self,
        file_path: &str,
        content: &str,
    ) -> Result<String, WorkspaceErrors> {
        let path = self.fs.resolve_write_path(file_path)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

        file.write_all(content.as_bytes())?;

        Ok(format!("Successfully appended to {}", file_path))
    }

    pub fn replace_in_file(
        &self,
        file_path: &str,
        old_text: &str,
        new_text: &str,
        replace_all: bool,
    ) -> Result<String, WorkspaceErrors> {
        let path = self.fs.resolve_safe_path(file_path)?;
        let content = fs::read_to_string(&path)?;

        if !content.contains(old_text) {
            return Err(WorkspaceErrors::new(
                format!("Text not found in file: {}", old_text),
                ErrorKind::Arguments,
            ));
        }

        let new_content = if replace_all {
            content.replace(old_text, new_text)
        } else {
            content.replacen(old_text, new_text, 1)
        };

        fs::write(&path, new_content)?;

        Ok(format!("Successfully replaced text in {}", file_path))
    }

    pub fn replace_in_multiple_files(
        &self,
        file_paths: &[String],
        old_text: &str,
        new_text: &str,
        replace_all: bool,
    ) -> Result<HashMap<String, String>, WorkspaceErrors> {
        let mut results = HashMap::with_capacity(file_paths.len());

        for file_path in file_paths {
            results.insert(
                file_path.clone(),
                self.replace_in_file(file_path, old_text, new_text, replace_all)?,
            );
        }

        Ok(results)
    }
}
