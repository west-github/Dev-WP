from abc import ABC, abstractmethod
from dataclasses import dataclass


class ICollector(ABC):
    @abstractmethod
    def give_in(self, walk: "Walk") -> None: ...

    @abstractmethod
    def give_data(self) -> str: ...


class Walk:

    def walk(self, collector: ICollector) -> None:
        print(f"Walking the thing here: {collector.give_data().upper()}")


@dataclass(kw_only=True)
class File(ICollector):

    path: str

    def give_in(self, walk: Walk) -> None:
        walk.walk(self)

    def give_data(self) -> str:
        return f"File = {self.path}"


@dataclass(kw_only=True)
class Text(ICollector):

    content: bytes

    def give_in(self, walk: Walk) -> None:
        walk.walk(self)

    def give_data(self) -> str:
        return f"File = {self.content.decode()}"


[
    walker.give_in(Walk())
    for walker in [File(path="./index.txt"), Text(content=b"Soldiers are coming")]
]
