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

    def write_multiple_files(self, files: Dict[str, str]) -> Dict[str, str]:
        results = {}

        for file_path, content in files.items():
            results[file_path] = self.write_file(file_path, content)
        return results

    def append_to_file(self, file_path: str, content: str) -> str:
        try:
            path = self.fs.resolve_write_path(file_path)
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open('a', encoding='utf-8') as f:
                f.write(content)
            return f"Successfully appended to {file_path}"
        except Exception as e:
            return f"Error appending to file: {str(e)}"

    