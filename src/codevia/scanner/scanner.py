from pathlib import Path
class Scanner:
    def __init__(self, project_path: str):
        self.project_path = project_path
    
    def scan_directory(self):
        root = Path(self.project_path)

        count = 0

        for file in root.rglob("*"):
            if any(part.startswith(".") for part in file.parts):
                continue
            if file.is_file():
                try:
                    content = file.read_text(encoding="utf-8")
                    print(content)
                except UnicodeDecodeError:
                    print(f"Skipping binary file: {file}")

        print(f"Files Scanned: {count}")

if __name__ == "__main__":
    scanner = Scanner("/home/scrom/code/beepitt")
    scanner.scan_directory()