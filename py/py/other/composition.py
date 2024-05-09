from abc import ABC, abstractmethod


class Service(ABC):
    @abstractmethod
    def compose(self) -> None: ...


class Foo(Service):
    def __init__(self) -> None: ...
    def compose(self) -> None:
        print("Yes this is a foo service")


class Client:
    service: Service

    def __init__(self) -> None:
        self.service = Foo()
        pass

    def use_compose(self) -> None:
        self.service.compose()


if __name__ == "__main__":
    client = Client()
    client.use_compose()
    ...
