from pathlib import Path
import os

class WorkspaceTools:
    def __init__(self):
        self.cwd = self._get_cwd()
        self.excluded_dirs = { ".git", ".venv", "venv", "__pycache__", ".pytest_cache", ".mypy_cache", ".ruff_cache", "node_modules", "dist", "build", ".next", ".cache"}

    def _get_cwd(self) -> str:
        return os.getcwd()

    def read_file(self, file_path: str) -> str:
        file_path_with_cwd = Path(self.cwd) / file_path.lstrip("/")

        try:
            with open(file_path_with_cwd, "r", encoding="utf-8") as file:
                return file.read()
        except Exception as e:
            return f"An error occurred: {str(e)}"

    def list_files(self, path: str = ".", recursive: bool = False) -> list[str]:
        directory = Path(self.cwd) / path.lstrip("/")

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


    def list_directories(self, path: str = ".", recursive: bool = False) -> list[str]:
        directory = Path(self.cwd) / path.lstrip("/")

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
    

if __name__ == "__main__":
    tools = WorkspaceTools()
    print(tools.list_files(recursive=False))
    print(tools.list_files(recursive=True))
    print(tools.list_directories(recursive=False))
    print(tools.list_directories(recursive=True))