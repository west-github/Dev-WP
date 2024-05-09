from abc import ABC, abstractmethod


class Service(ABC):
    @abstractmethod
    def service(self) -> None:
        ...


class ClientService(Service):
    def service(self) -> None:
        print("This is injected and implemented the service interface")


class Client:
    _service: Service

    def __init__(self, service: Service) -> None:
        self._service = service

    def do_something(self) -> None:
        self._service.service()


def di_init() -> None:
    #

    if True:
        client = Client(ClientService())

        client.do_something()
