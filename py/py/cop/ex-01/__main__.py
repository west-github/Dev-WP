from abc import ABC, abstractmethod
from typing import Self


class IHandler(ABC):
    next: Self | None = None

    def set_next(self, handler: Self) -> Self:
        self.next = handler
        return handler

    @abstractmethod
    def handle[T](self, payload: T) -> None:
        return self.next and self.next.handle(payload)


class Monkey(IHandler):
    value: int

    def __init__(self, value: int, handler: IHandler | None = None) -> None:
        if handler:
            self.next = handler
        self.value = value

    def handle[T](self, request: T) -> None:
        print(f"This is a monkey class {request} from {self.value}")
        return super().handle(request)


class Dog(IHandler):
    value: int

    def __init__(self, value: int, handler: IHandler | None = None) -> None:
        if handler:
            self.next = handler
        self.value = value

    def handle[T](self, request: T) -> None:
        print(f"This is a dog class {request} from {self.value}")
        return None


if __name__ == "__main__":
    Monkey(1, Dog(2, Monkey(3))).handle(["Nut", "Banana", "Orange"])
