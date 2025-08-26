class MyClass:
    """Docstring for MyClass."""
    def __new__(cls, name: str, x: float, y: float):
        """Docstring for new()."""
        ...

    def print_list(self, lst: list[int]) -> str | None:
        """Prints elements of the list."""
        ...

    def show(self) -> str:
        """Prints instance as string."""
        ...

def hello_from_bin() -> str:
    """Another docstring."""
    ...
