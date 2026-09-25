from pathlib import Path
import os

class WorkspaceTools:
    def __init__(self):
        self.cwd = self._get_cwd()

    def _get_cwd(self) -> str:
        return os.getcwd()

    def read_file(self, file_path: str) -> str:
        file_path_with_cwd = Path(self.cwd) / file_path.lstrip("/")

        try:
            with open(file_path_with_cwd, "r", encoding="utf-8") as file:
                return file.read()
        except Exception as e:
            return f"An error occurred: {str(e)}"

    def list_files(self, path: str = ".") -> list[str]:
        directory = Path(self.cwd) / path.lstrip("/")

        return [item.name for item in directory.iterdir() if item.is_file()]

    def list_directories(self, path: str = ".") -> list[str]:
        directory = Path(self.cwd) / path.lstrip("/")

        return [item.name for item in directory.iterdir() if item.is_dir()]

    

if __name__ == "__main__":
    tools = WorkspaceTools()
    tools.list_files("")
    tools.list_directories("")