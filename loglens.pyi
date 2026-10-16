from pathlib import Path

class Report:
    @property
    def content(self) -> str: ...


def analyze(path: str | Path) -> Report: ...
