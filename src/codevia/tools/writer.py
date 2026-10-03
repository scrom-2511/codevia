from pathlib import Path
from typing import Dict, Optional
from codevia.tools.file_system import FileSystem

class Writer:
    def __init__(self, fs: FileSystem):
        self.fs = fs

    def write_file(self, file_path: str, content: str) -> str:
        try:
            path = self.fs.resolve_write_path(file_path)
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding='utf-8')
            return f"Successfully wrote to {file_path}"
        except Exception as e:
            return f"Error writing file: {str(e)}"