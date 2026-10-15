use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::core::WorkspaceErrors;
use crate::core::tools::file_system::FileSystem;

pub struct Reader {
    fs: FileSystem,
}

impl Reader {
    pub fn new(fs: FileSystem) -> Self {
        Self { fs }
    }

    pub fn read_file(&self, file_path: &str, start: Option<usize>, end: Option<usize>) -> String {
        match self.try_read_file(file_path, start, end) {
            Ok(content) => content,
            Err(err) => err.to_string(),
        }
    }

    fn try_read_file(
        &self,
        file_path: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<String, WorkspaceErrors> {
        let safe_path = self
            .fs
            .resolve_safe_path(file_path)
            .map_err(|e| e.to_string())?;

        let file = File::open(&safe_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader
            .lines()
            .collect::<Result<_, _>>()
            .map_err(|e| e.to_string())?;

        let start_idx = start.unwrap_or(1);
        let end_idx = end.unwrap_or(lines.len());

        if start_idx < 1 {
            return Err(WorkspaceErrors::Arguments("start must be >= 1".to_string()));
        }

        if end_idx < start_idx {
            return Err(WorkspaceErrors::Arguments(
                "end must be >= start".to_string(),
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
