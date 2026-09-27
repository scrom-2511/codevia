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

    def read_file(self, file_path: str) -> str:
        try:
            file_path_with_cwd = self._resolve_safe_path(file_path)

            with open(file_path_with_cwd, "r", encoding="utf-8") as file:
                return file.read()

        except Exception as e:
            return str(e)

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
    

if __name__ == "__main__":
    tools = WorkspaceTools()
    print(tools.read_file("../env"))
    print(tools.list_files(recursive=False))
    print(tools.list_files(recursive=True))
    print(tools.list_directories(recursive=False))
    print(tools.list_directories(recursive=True))