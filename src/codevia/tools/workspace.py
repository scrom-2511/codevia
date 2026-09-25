import os

class WorkspaceTools:
    def __init__(self):
        self.cwd = self._get_cwd()

    def _get_cwd(self):
        return os.getcwd()

if __name__ == "__main__":
    tools = WorkspaceTools()