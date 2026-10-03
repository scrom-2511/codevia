from codevia.tools.file_system import FileSystem

class Reader:
    def __init__(self, fs:FileSystem):
        self.fs = fs

    def read_file(self, file_path: str, start: int|None = None, end: int|None = None) -> str:
        try:
            file_path_with_cwd = self.fs.resolve_safe_path(file_path)

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