from codevia.tools.file_system import FileSystem
from codevia.tools.reader import Reader
from codevia.tools.discovery import Discovery
from codevia.tools.code_intelligence import CodeIntelligence
from codevia.tools.writer import Writer

class WorkspaceTools:
    def __init__(self):
        self.fs = FileSystem()
        self.reader = Reader(self.fs)
        self.discovery = Discovery(self.fs)
        self.code = CodeIntelligence(self.fs)
        self.writer = Writer(self.fs)

    # Reader
    def read_file(self, *args, **kwargs):
        return self.reader.read_file(*args, **kwargs)

    def read_multiple_files(self, *args, **kwargs):
        return self.reader.read_multiple_files(*args, **kwargs)

    # Discovery
    def list_files(self, *args, **kwargs):
        return self.discovery.list_files(*args, **kwargs)

    def list_directories(self, *args, **kwargs):
        return self.discovery.list_directories(*args, **kwargs)

    def list_files_and_directories(self, *args, **kwargs):
        return self.discovery.list_files_and_directories(*args, **kwargs)

    def find_files(self, *args, **kwargs):
        return self.discovery.find_files(*args, **kwargs)

    def get_file_info(self, *args, **kwargs):
        return self.discovery.get_file_info(*args, **kwargs)

    def search_in_files(self, *args, **kwargs):
        return self.discovery.search_in_files(*args, **kwargs)

    def search_multiple_patterns(self, *args, **kwargs):
        return self.discovery.search_multiple_patterns(*args, **kwargs)

    # Code intelligence
    def get_file_outline(self, *args, **kwargs):
        return self.code.get_file_outline(*args, **kwargs)

    def get_multiple_files_outline(self, *args, **kwargs):
        return self.code.get_multiple_files_outline(*args, **kwargs)

    # Writer
    def write_file(self, *args, **kwargs):
        return self.writer.write_file(*args, **kwargs)

    def append_to_file(self, *args, **kwargs):
        return self.writer.append_to_file(*args, **kwargs)

    def replace_in_file(self, *args, **kwargs):
        return self.writer.replace_in_file(*args, **kwargs)

    def write_multiple_files(self, *args, **kwargs):
        return self.writer.write_multiple_files(*args, **kwargs)

    def replace_in_multiple_files(self, *args, **kwargs):
        return self.writer.replace_in_multiple_files(*args, **kwargs)

    # File system
    def create_file(self, *args, **kwargs):
        return self.fs.create_file(*args, **kwargs)


if __name__ == "__main__":
    tools = WorkspaceTools()
    # print(tools.reader.read_file("./.env", 1, 2))
    # print(tools.discovery.list_files(recursive=False))
    # print(tools.discovery.list_files(recursive=True))
    # print(tools.discovery.list_directories(recursive=False))
    # print(tools.discovery.list_directories(recursive=True))
    # print(tools.discovery.search_in_files(pattern=r"^\s*import\s+.+$", path="./", before_context=2, after_context=2))
    print(tools.code.get_file_outline("./src/codevia/tools/workspace.py"))