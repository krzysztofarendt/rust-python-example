from rustpy._core import hello_from_bin
from rustpy._core import MyClass

def main() -> None:
    print(hello_from_bin())

    c = MyClass("I'm alive!", x=1.5, y=9.9)
    print(c.show())
    print(c.print_list([1, 2, 3, 4, 5]))
