from pathlib import Path
import os

class FileSystem:
    def __init__(self):
        self.cwd = self._get_cwd()
        self.excluded_dirs = { ".git", ".venv", "venv", "__pycache__", ".pytest_cache", ".mypy_cache", ".ruff_cache", "node_modules", "dist", "build", ".next", ".cache"}

    def _get_cwd(self) -> Path:
        return Path(os.getcwd()).resolve()

    def resolve_safe_path(self, path: str) -> Path:
        target_path = (Path(self.cwd) / path.lstrip("/")).resolve()

        if not target_path.is_relative_to(self.cwd):
            raise PermissionError(
                f"Access denied: '{path}' is outside the workspace."
            )
        
        if not target_path.exists():
            raise FileNotFoundError(f"Path not found: '{path}'")

        return target_path
