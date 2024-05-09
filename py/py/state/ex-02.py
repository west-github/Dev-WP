from abc import ABC, abstractmethod
from time import sleep
from typing import Any


class State(ABC):

    @abstractmethod
    def handle(self, context: Any) -> None: ...


class Context:
    _state: State

    def __init__(self, state: State) -> None:
        self._state = state

    @property
    def state(self) -> State:
        return self._state

    @state.setter
    def state(self, state: State) -> None:
        self._state = state

    def handle(self) -> None:
        self._state.handle(self)


class Unpaid(State):

    def handle(self, context: Context) -> None:
        print("Receiving payment getting to work now")
        context.state = Paid()


class Paid(State):

    def handle(self, context: Any) -> None:
        print("Using payment now done using payment")
        context.state = Unpaid()


if __name__ == "__main__":

    ctx = Context(Paid())

    while True:
        ctx.handle()

        sleep(2)
