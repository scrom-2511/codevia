import subprocess
import platform
import shutil
from pathlib import Path
import os

class WorkspaceTools:
    def __init__(self):
        self.cwd = self._get_cwd()
        self.excluded_dirs = { ".git", ".venv", "venv", "__pycache__", ".pytest_cache", ".mypy_cache", ".ruff_cache", "node_modules", "dist", "build", ".next", ".cache"}

    def _get_cwd(self) -> Path:
        return Path(os.getcwd()).resolve()

    def _resolve_safe_path(self, path: str) -> Path:
        target_path = (Path(self.cwd) / path.lstrip("/")).resolve()

        if not target_path.is_relative_to(self.cwd):
            raise PermissionError(
                f"Access denied: '{path}' is outside the workspace."
            )
        
        if not target_path.exists():
            raise FileNotFoundError(f"Path not found: '{path}'")

        return target_path

    def read_file(self, file_path: str, start: int|None = None, end: int|None = None) -> str:
        try:
            file_path_with_cwd = self._resolve_safe_path(file_path)

            with open(file_path_with_cwd, "r", encoding="utf-8") as file:
                lines = file.readlines()

                start = 1 if start is None else start
                end = len(lines) if end is None else end

                if start < 1:
                    raise ValueError("start must be >= 1")

                if end < start:
                    raise ValueError("end must be >= start")

                selected = lines[start - 1:end]

                final_str = ""
                
                for i, str in enumerate(selected, start=start):
                    final_str += (f"{i}: {str}")

                return final_str

        except Exception as e:
            return str(e)

    def read_multiple_files(self, file_paths: list[str], start: int | None = None, end: int | None = None) -> dict[str, str]:
        results = {}
        for file_path in file_paths:
            results[file_path] = self.read_file(
                file_path=file_path,
                start=start,
                end=end
            )
        return results

    def list_files(self, path: str = ".", recursive: bool = False) -> list[str] | str:
        try:
            directory = self._resolve_safe_path(path)

            if not recursive:
                return [
                    str(item.relative_to(directory))
                    for item in directory.iterdir()
                    if item.is_file()
                ]

            result = []

            for root, dirs, files in os.walk(directory):
                dirs[:] = [d for d in dirs if d not in self.excluded_dirs]

                for filename in files:
                    item = Path(root) / filename
                    result.append(str(item.relative_to(directory)))

            return result

        except Exception as e:
            return str(e)

    def list_directories(self, path: str = ".", recursive: bool = False) -> list[str] | str:
        try:
            directory = self._resolve_safe_path(path)

            if not recursive:
                return [
                    str(item.relative_to(directory))
                    for item in directory.iterdir()
                    if item.is_dir() and item.name not in self.excluded_dirs
                ]

            result = []

            for root, dirs, files in os.walk(directory):
                dirs[:] = [d for d in dirs if d not in self.excluded_dirs]

                for dirname in dirs:
                    item = Path(root) / dirname
                    result.append(str(item.relative_to(directory)))

            return result

        except Exception as e:
            return str(e)
    
    def list_files_and_directories(self, path: str = ".", recursive: bool = False) -> dict[str, list[str]]:
        try:
            files = self.list_files(path, recursive)
            directories = self.list_directories(path, recursive)

            return {"files": files, "directories": directories}

        except Exception as e:
            return str(e)

    def find_files(self, pattern: str, path: str = ".") -> str:
        if not self.ensure_rg():
            return "ripgrep is not installed."

        try:
            path_obj = self._resolve_safe_path(path)
            
            command = ["rg", "--files", "-g", pattern, str(path_obj)]

            result = subprocess.run(command, capture_output=True, text=True)

            if result.returncode == 1:
                return "No files found matching the pattern."

            if result.returncode != 0:
                return f"ripgrep error:\n{result.stderr}"

            return result.stdout

        except Exception as e:
            return str(e)

    def get_file_info(self, file_path: str) -> dict | str:
        try:
            path_obj = self._resolve_safe_path(file_path)
            stats = path_obj.stat()
            return {
                "name": path_obj.name,
                "size_bytes": stats.st_size,
                "created_time": stats.st_ctime,
                "modified_time": stats.st_mtime,
                "is_file": path_obj.is_file(),
                "is_dir": path_obj.is_dir()
            }
        except Exception as e:
            return str(e)

    def ensure_rg(self) -> bool:
        if shutil.which("rg"):
            return True

        system = platform.system()

        if system == "Linux":
            command = ["sudo", "apt", "install", "-y", "ripgrep"]

        elif system == "Darwin":
            command = ["brew", "install", "ripgrep"]

        elif system == "Windows":
            command = ["winget", "install", "--id", "BurntSushi.ripgrep.MSVC"]

        else:
            return False

        print("ripgrep is required for file search.")
        print(f"Install command: {' '.join(command)}")

        answer = input("Install it? [y/N] ")

        if answer.lower() != "y":
            return False

        result = subprocess.run(command)

        return result.returncode == 0

    def search_in_files(
        self,
        pattern: str,
        path: str = ".",
        before_context: int = 0,
        after_context: int = 0,
        multiline: bool = False,
    ) -> str:
        if not self.ensure_rg():
            return "ripgrep is not installed."

        try:
            path = self._resolve_safe_path(path)
            
            command = ["rg", "-n"]

            if before_context > 0:
                command.extend(["-B", str(before_context)])

            if after_context > 0:
                command.extend(["-A", str(after_context)])

            if multiline:
                command.append("-U")

            command.extend(["--", pattern, path])

            result = subprocess.run(command, capture_output=True, text=True)

            if result.returncode == 1:
                return "No matches found."

            if result.returncode != 0:
                return f"ripgrep error:\n{result.stderr}"

            return result.stdout

        except Exception as e:
            return str(e)

    def search_multiple_patterns(
        self,
        patterns: list[str],
        path: str = ".",
        before_context: int = 0,
        after_context: int = 0,
        multiline: bool = False,
    ) -> dict[str, str]:
        results = {}
        for pattern in patterns:
            results[pattern] = self.search_in_files(
                pattern=pattern,
                path=path,
                before_context=before_context,
                after_context=after_context,
                multiline=multiline
            )
        return results

if __name__ == "__main__":
    tools = WorkspaceTools()
    # print(tools.read_file("./.env", 1, 2))
    # print(tools.list_files(recursive=False))
    # print(tools.list_files(recursive=True))
    # print(tools.list_directories(recursive=False))
    # print(tools.list_directories(recursive=True))
    # print(tools.search_in_files(pattern=r"^\s*import\s+.+$", path="./", before_context=2, after_context=2))