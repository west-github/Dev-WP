from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Callable, Self


@dataclass
class User:
    name: str
    age: int

    def layer(self, func: Callable[[Self, Callable[[], Self]], None]) -> None: ...


# @dataclass(init=False)
class Handler(ABC):

    _next: Self | None = None

    def set_next(self, next: Self) -> Self:
        self._next = next
        return next

    @abstractmethod
    def handle(self, payload: User) -> None:
        return self._next.handle(payload) if self._next else None


# class Layer(Handler):

#     def handle(self, payload: User) -> None:
#         return super().handle(payload)


# @dataclass(init=False)
class Print(Handler):

    def handle(self, payload: User) -> None:
        print(f"This is a user with name: {payload.name} and age {payload.age}")

        return super().handle(payload)


# @dataclass(init=False)
class Logger(Handler):

    def handle(self, payload: User) -> None:
        print(f"logging  user with name: {payload.name} and age {payload.age}")

        return super().handle(payload)


if __name__ == "__main__":
    User("west", 30)

    req = Print()
    logger = Logger()
    _print = Print()

    # print(asdict(req))

    pass
