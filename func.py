"""Shared utility functions."""


def print_log(message: str) -> None:
    """Print a log message to standard output."""
    print(message)


def debug(message: str) -> None:
    """Print a debug message using the shared logging utility."""
    print_log(message)
