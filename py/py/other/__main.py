from dataclasses import dataclass
from typing import Callable
from py.other.di import di_init
from strategy.example_001 import strategy_ex_001
from dataclasses import dataclass, field
from typing import Any


@dataclass()
class User:
    # __slots__ = ("name", "age")

    name: str = field(kw_only=True)
    age: int = field(default_factory=lambda: 20)

    is_valid: bool = field(default=False)

    def is_valid_check(self) -> bool:
        return "@" not in self.name and self.age >= 20


class Profile:
    __slots__ = "name"

    __name = "West"

    name: str

    def __init__(self, name: str) -> None:
        self.name = name


def kw_args(*args: Any, **kwargs: Any) -> None:

    for arg in args:
        print(arg)

    for kw in kwargs:
        print(kw)


def n_func(name: str, *, age: int) -> None:
    print("*" * 10)
    print(name, age)
    print("*" * 10)


@dataclass
class Data:
    __slot__ = ("value", "bytes")
    value: str
    bytes: bytes

    def do_something(self, predicate: Callable[[], bool]) -> None:
        assert callable(predicate)

        if predicate():
            print(self.value)


@dataclass
class Foo:
    foo: str

    def use_foo(self) -> None:
        print(f"{self.foo}")


class Bar(Data, Foo):
    def __init__(self, value: str, bytes: bytes, foo: str) -> None:
        self.foo = foo

        super().__init__(value, bytes)

    def use_super(self) -> None:
        self.use_foo()

        print(f"{self.foo} {self.value} {self.bytes}")

        self.do_something(lambda: True)


class PositiveInteger(int):
    """
    What if we can be executed as python
    """

    def __new__(cls, value: int):
        print("New called")

        if value >= 0:
            return super().__new__(cls, value)
        else:
            raise ValueError("PositiveInteger must be non-negative")


if __name__ == "__main__":
    foo = "Some value"
    match foo:
        case "Some value":
            print("Yes some value")

    if False:
        positive_int = PositiveInteger(20)

        print(positive_int)

        print(positive_int.__dict__, positive_int.__doc__)

        data = Data(value="West", bytes=bytes(1))

        data.do_something(lambda: True)

        def multi_process_thread() -> None:
            """
            Pool
            Queue
            Thread
            Process
            """
            ...

        def test_func() -> None:
            """
            Testing
            """
            print("Testing List With Multiplication")

        test_funcs = [test_func] * 5 + [test_func] * 2

        test_funcs *= 2

        [test_func() for test_func in test_funcs if test_func() is not None]

        list = [1, 2, 3, 4, 5]

        list += [6, 7, 8, 9, 10]

        list.extend([6, 7, 8, 9, 10])

        print(list)

        multiple_inheritance = Bar("Some Value", bytes(20), "Some")

        multiple_inheritance.use_super()

        # Dependency Injection
        di_init()

        # Strategy Pattern
        strategy_ex_001()

        # State Pattern
        state_ex_001()

        # Abstract class Method
        abstract_class_method_init()
