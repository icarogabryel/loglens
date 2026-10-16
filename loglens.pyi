from pathlib import Path
from typing import Literal

from typing_extensions import TypeAlias

LogLevel: TypeAlias = Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]


class LogRecord:
    @property
    def timestamp(self) -> str: ...

    @property
    def level(self) -> LogLevel: ...

    @property
    def logger(self) -> str: ...

    @property
    def message(self) -> str: ...


class Report:
    @property
    def records(self) -> list[LogRecord]: ...

    def print(self) -> None: ...


def analyze(path: str | Path) -> Report: ...
