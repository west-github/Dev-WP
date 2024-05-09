from abc import ABC, abstractmethod
from typing import Callable


class Button(ABC):
    def on_click(self, func: Callable[[], None]) -> None:
        func()

    def render(self) -> None:
        print(f"Rendering... {self.__class__.__name__}")


class Dialog(ABC):
    button: Button

    def __init__(self, button: Button) -> None:
        super().__init__()
        self.button = button

    def render(self) -> None:
        self.button.on_click(self.close)
        self.button.render()

    @abstractmethod
    def close(self) -> None: ...


class WindowDialog(Dialog):
    def __init__(self) -> None:
        super().__init__(WindowButton())

    def close(self) -> None:
        print(f"The {self.__class__.__name__} close function is clicked")


class MacDialog(Dialog):
    def __init__(self) -> None:
        super().__init__(MacButton())

    def close(self) -> None:
        print(f"The {self.__class__.__name__} close function is clicked")


class WindowButton(Button): ...


class MacButton(Button): ...


if __name__ == "__main__":
    os = input("Enter the os: ").lower()

    dialog = WindowDialog() if os == "window" else MacDialog()

    dialog.render()
