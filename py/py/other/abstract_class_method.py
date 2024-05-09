from abc import ABC, abstractmethod
from dataclasses import dataclass


class Shape(ABC):
    class_variable = "What's Good"

    @abstractmethod
    @classmethod
    def cls_var(cls, args: str) -> None: ...


@dataclass
class Rectangle(Shape):
    width: int
    height: int

    def area(self) -> int:
        return self.width * self.height

    @classmethod
    def cls_var(cls, args: str) -> None:
        print(f"{cls.class_variable} is from parent with args {args}")

    # @classmethod
    # def __subclasshook__(cls, __subclass: type) -> bool:
    #     return super().__subclasshook__(__subclass)


def abstract_class_method_init() -> None:
    rect = Rectangle(20, 30)
    rect.area()
    Rectangle.cls_var("Wow got it")
