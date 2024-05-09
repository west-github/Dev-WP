from abc import ABC, abstractmethod


class Service(ABC):
    @abstractmethod
    def service(self) -> None: ...


class ClientService(Service):
    def service(self) -> None:
        msg = "This is injected and implemented the service interface strategy pattern"
        print(msg)


class OtherClientService(Service):
    def service(self) -> None:
        msg = "This is other injected  and implemented the service interface strategy pattern"
        print(msg)


class Client:
    _service: Service

    def __init__(self, service: Service) -> None:
        self._service = service

    def set_service(self, value: Service) -> None:
        assert isinstance(value, Service)
        self._service = value

    def do_something(self) -> None:
        self._service.service()


def strategy_ex_001() -> None:
    #

    if True:
        client = Client(ClientService())
        client.do_something()
        client.set_service(OtherClientService())
        client.do_something()
