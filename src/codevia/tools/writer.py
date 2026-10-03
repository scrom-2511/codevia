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

    def replace_in_file(self, file_path: str, old_text: str, new_text: str, replace_all: bool = False) -> str:
        try:
            path = self.fs.resolve_safe_path(file_path)
            content = path.read_text(encoding='utf-8')
            
            if old_text not in content:
                raise ValueError(f"Text not found in file: {old_text}")

            if replace_all:
                content = content.replace(old_text, new_text)
            else:
                content = content.replace(old_text, new_text, 1)
                
            path.write_text(content, encoding='utf-8')
            return f"Successfully replaced text in {file_path}"
        except Exception as e:
            return f"Error replacing text in file: {str(e)}"

    def replace_in_multiple_files(self, file_paths: list[str], old_text: str, new_text: str, replace_all: bool = False) -> Dict[str, str]:
        results = {}
        
        for file_path in file_paths:
            results[file_path] = self.replace_in_file(file_path, old_text, new_text, replace_all)
        return results
