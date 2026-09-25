import subprocess
class WorkspaceTools:
    def __init__(self):
        self.cwd = self._get_cwd()

    def _get_cwd(self):
        output = subprocess.run("pwd", shell=True, capture_output=True, text=True, timeout=30)
        return output.stdout + output.stderr

if __name__ == "__main__":
    tools = WorkspaceTools()