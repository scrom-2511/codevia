use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::core::ErrorKind;
use crate::core::tools::WorkspaceErrors;
use crate::core::tools::file_system::FileSystem;

#[derive(Debug, Clone, Copy)]
pub struct Reader<'a> {
    fs: &'a FileSystem,
}

impl<'a> Reader<'a> {
    pub fn new(fs: &'a FileSystem) -> Self {
        Self { fs }
    }

    pub fn read_file(&self, file_path: &str, start: Option<usize>, end: Option<usize>) -> String {
        match self.try_read_file(file_path, start, end) {
            Ok(content) => content,
            Err(err) => err.to_string(),
        }
    }

    pub fn try_read_file(
        &self,
        file_path: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<String, WorkspaceErrors> {
        let safe_path = self.fs.resolve_safe_path(file_path)?;

        let file = File::open(&safe_path)?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        let start_idx = start.unwrap_or(1);
        let end_idx = end.unwrap_or(lines.len());

        if start_idx < 1 {
            return Err(WorkspaceErrors::new(
                "start must be >= 1",
                ErrorKind::Arguments,
            ));
        }

        if end_idx < start_idx {
            return Err(WorkspaceErrors::new(
                "end must be >= start",
                ErrorKind::Arguments,
            ));
        }

        let from = (start_idx - 1).min(lines.len());
        let to = end_idx.min(lines.len());

        let mut output = String::new();
        for (offset, line) in lines[from..to].iter().enumerate() {
            let line_num = start_idx + offset;
            let _ = writeln!(output, "{}: {}", line_num, line);
        }

        Ok(output)
    }

    pub fn read_multiple_files(
        &self,
        file_paths: &[String],
        start: Option<usize>,
        end: Option<usize>,
    ) -> HashMap<String, String> {
        let mut results = HashMap::with_capacity(file_paths.len());

        for path in file_paths {
            let content = self.read_file(path, start, end);
            results.insert(path.clone(), content);
        }

        results
    }
}
