from typing import Any
from time import sleep
from abc import ABC, abstractmethod


class State(ABC):
    _app: Any | None = None

    @property
    def app(self) -> Any:
        return self._app

    @app.setter
    def app(self, app: Any) -> None:
        self._app = app

    @abstractmethod
    def work(self) -> None: ...


class Unpaid(State):

    def work(self) -> None:
        print("Yes just got paid let work")
        self.app.set_state(Paid())


class Paid(State):

    def work(self) -> None:
        print("Work i'm paid so i'm working")
        self.app.set_state(Unpaid())


class Application:
    _state: State

    def __init__(self, state: State) -> None:
        self.set_state(state)

    def set_state(self, state: State) -> None:
        self._state = state
        self._state.app = self

    def work(self) -> None:
        self._state.work()


if __name__ == "__main__":

    app = Application(Paid())

    while True:
        app.work()

        sleep(1)
