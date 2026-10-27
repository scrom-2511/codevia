use std::collections::HashMap;

use serde_json::Value;

use crate::core::tools::{
    discovery::Discovery, file_system::FileSystem, reader::Reader, writer::Writer,
};

pub type ToolHandler<'a> = Box<dyn Fn(&Value) -> Result<String, String> + 'a>;

pub struct ToolRegistry<'a> {
    pub tools: HashMap<String, ToolHandler<'a>>,
}

impl<'a> ToolRegistry<'a> {
    pub fn new(fs: &'a FileSystem) -> Self {
        let mut registry = Self::empty();
        registry.register_default_tools(fs);
        registry
    }

    pub fn empty() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: impl Into<String>, handler: F)
    where
        F: Fn(&Value) -> Result<String, String> + 'a,
    {
        self.tools.insert(name.into(), Box::new(handler));
    }

    pub fn insert(&mut self, name: impl Into<String>, handler: ToolHandler<'a>) {
        self.tools.insert(name.into(), handler);
    }

    pub fn execute(&self, name: &str, args: &Value) -> Result<String, String> {
        match self.tools.get(name) {
            Some(handler) => handler(args),
            None => Err(format!("Unknown tool: '{name}'")),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn register_default_tools(&mut self, fs: &'a FileSystem) {
        let discovery = Discovery::new(fs);
        let reader = Reader::new(fs);
        let writer = Writer::new(fs);

        self.register("list_files", move |args| {
            let path = get_str_arg(args, "path")?;
            let recursive = get_bool_arg(args, "recursive", false)?;
            let files = discovery
                .list_files(path, recursive)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&files).map_err(|e| e.to_string())
        });

        self.register("list_directories", move |args| {
            let path = get_str_arg(args, "path")?;
            let recursive = get_bool_arg(args, "recursive", false)?;
            let directories = discovery
                .list_directories(path, recursive)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&directories).map_err(|e| e.to_string())
        });

        self.register("list_files_and_directories", move |args| {
            let path = get_str_arg(args, "path")?;
            let recursive = get_bool_arg(args, "recursive", false)?;
            let entries = discovery
                .list_files_and_directories(path, recursive)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&entries).map_err(|e| e.to_string())
        });

        self.register("find_files", move |args| {
            let pattern = get_str_arg(args, "pattern")?;
            let path = get_str_arg(args, "path")?;
            discovery
                .find_files(pattern, path)
                .map_err(|e| e.to_string())
        });

        self.register("get_file_info", move |args| {
            let file_path = get_str_arg(args, "file_path")?;
            let info = discovery
                .get_file_info(file_path)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&info).map_err(|e| e.to_string())
        });

        self.register("search_in_files", move |args| {
            let pattern = get_str_arg(args, "pattern")?;
            let path = get_str_arg(args, "path")?;
            let before_context = get_usize_arg(args, "before_context")?.unwrap_or(0);
            let after_context = get_usize_arg(args, "after_context")?.unwrap_or(0);
            let multiline = get_bool_arg(args, "multiline", false)?;
            discovery
                .search_in_files(pattern, path, before_context, after_context, multiline)
                .map_err(|e| e.to_string())
        });

        self.register("search_multiple_patterns", move |args| {
            let patterns = args
                .get("patterns")
                .and_then(Value::as_array)
                .ok_or("Missing or invalid 'patterns' parameter")?
                .iter()
                .map(|v| {
                    v.as_str()
                        .map(String::from)
                        .ok_or("Pattern must be a string")
                })
                .collect::<Result<Vec<String>, &'static str>>()?;
            let path = get_str_arg(args, "path")?;
            let before_context = get_usize_arg(args, "before_context")?.unwrap_or(0);
            let after_context = get_usize_arg(args, "after_context")?.unwrap_or(0);
            let multiline = get_bool_arg(args, "multiline", false)?;
            let results = discovery
                .search_multiple_patterns(&patterns, path, before_context, after_context, multiline)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&results).map_err(|e| e.to_string())
        });

        self.register("create_file", move |args| {
            let file_path = get_str_arg(args, "file_path")?;
            fs.try_create_file(file_path).map_err(|e| e.to_string())
        });

        self.register("read_file", move |args| {
            let file_path = get_str_arg(args, "file_path")?;
            let start = get_usize_arg(args, "start")?;
            let end = get_usize_arg(args, "end")?;
            reader
                .try_read_file(file_path, start, end)
                .map_err(|e| e.to_string())
        });

        self.register("read_multiple_files", move |args| {
            let file_paths = args
                .get("file_paths")
                .and_then(Value::as_array)
                .ok_or("Missing or invalid 'file_paths' parameter")?
                .iter()
                .map(|v| {
                    v.as_str()
                        .map(String::from)
                        .ok_or("File path must be a string")
                })
                .collect::<Result<Vec<String>, &'static str>>()?;
            let start = get_usize_arg(args, "start")?;
            let end = get_usize_arg(args, "end")?;
            let results = reader.read_multiple_files(&file_paths, start, end);
            serde_json::to_string_pretty(&results).map_err(|e| e.to_string())
        });

        self.register("write_file", move |args| {
            let file_path = get_str_arg(args, "file_path")?;
            let content = get_str_arg(args, "content")?;
            writer
                .write_file(file_path, content)
                .map_err(|e| e.to_string())
        });

        self.register("write_multiple_files", move |args| {
            let files_map = args
                .get("files")
                .and_then(Value::as_object)
                .ok_or("Missing or invalid 'files' parameter")?;
            let mut files = HashMap::with_capacity(files_map.len());
            for (k, v) in files_map {
                let content = v.as_str().ok_or("File content must be a string")?;
                files.insert(k.clone(), content.to_string());
            }
            let results = writer
                .write_multiple_files(&files)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&results).map_err(|e| e.to_string())
        });

        self.register("append_to_file", move |args| {
            let file_path = get_str_arg(args, "file_path")?;
            let content = get_str_arg(args, "content")?;
            writer
                .append_to_file(file_path, content)
                .map_err(|e| e.to_string())
        });

        self.register("replace_in_file", move |args| {
            let file_path = get_str_arg(args, "file_path")?;
            let old_text = get_str_arg(args, "old_text")?;
            let new_text = get_str_arg(args, "new_text")?;
            let replace_all = get_bool_arg(args, "replace_all", false)?;
            writer
                .replace_in_file(file_path, old_text, new_text, replace_all)
                .map_err(|e| e.to_string())
        });

        self.register("replace_in_multiple_files", move |args| {
            let file_paths = args
                .get("file_paths")
                .and_then(Value::as_array)
                .ok_or("Missing or invalid 'file_paths' parameter")?
                .iter()
                .map(|v| {
                    v.as_str()
                        .map(String::from)
                        .ok_or("File path must be a string")
                })
                .collect::<Result<Vec<String>, &'static str>>()?;
            let old_text = get_str_arg(args, "old_text")?;
            let new_text = get_str_arg(args, "new_text")?;
            let replace_all = get_bool_arg(args, "replace_all", false)?;
            let results = writer
                .replace_in_multiple_files(&file_paths, old_text, new_text, replace_all)
                .map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&results).map_err(|e| e.to_string())
        });
    }
}

fn get_str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("Missing or invalid '{key}' parameter"))
}

fn get_bool_arg(args: &Value, key: &str, default: bool) -> Result<bool, String> {
    match args.get(key) {
        Some(v) => v
            .as_bool()
            .ok_or_else(|| format!("Invalid '{key}' parameter, expected boolean")),
        None => Ok(default),
    }
}

fn get_usize_arg(args: &Value, key: &str) -> Result<Option<usize>, String> {
    match args.get(key) {
        Some(v) => {
            let num = v
                .as_u64()
                .ok_or_else(|| format!("Invalid '{key}' parameter, expected integer"))?;
            Ok(Some(num as usize))
        }
        None => Ok(None),
    }
}
