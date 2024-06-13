from dataclasses import dataclass
from typing import Any, Callable, Self


@dataclass(kw_only=True)
class User:
    name: str

    def __len__(self) -> int:
        return len(self.name)

    @classmethod
    def user_cls(cls, name: str) -> Self:
        return cls(name=name)

    def __str__(self) -> str:
        return f"{self.name}"


class West(User):

    def __repr__(self) -> str:
        return f"West(name='{self.name}')"

    def __str__(self) -> str:
        return self.__repr__()


def decor(func) -> Callable[[], None]:
    def wrapper(*args, **kwargs) -> None:
        print(f"Inside the decorator with args: {args[0]}")
        return func(*args, **kwargs)

    return wrapper


class other:

    def __init__(self, *, log: bool) -> None:
        self.log = log

    def __call__(self, func) -> Any:

        if self.log:
            print(f"This is decorator with args: {self.log}")

        def wrapper(*args, **kwargs):
            res = func(*args, **kwargs)
            print("This should print last")
            return res

        return wrapper


@other(log=True)
@decor
def use_decor(name: str) -> int:
    return len(name)


# Always true
if __name__ == "__main__":
    west = West.user_cls("East")
    print(f"{west}")

    use_decor("West is golden")
