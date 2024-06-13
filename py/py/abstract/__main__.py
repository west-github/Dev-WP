from abc import ABC, abstractmethod


class IPizza(ABC):

    @abstractmethod
    def prepare(self) -> None: ...


class Store(ABC):

    @abstractmethod
    def create_pizza(self) -> IPizza: ...


class Cheese(IPizza):

    def prepare(self) -> None:
        print("This is a cheese pizza")


class NewYork(Store):

    def create_pizza(self) -> IPizza:
        return Cheese()


def order_pizza(store: Store) -> None:
    pizza = store.create_pizza()

    pizza.prepare()


if __name__ == "__main__":
    order_pizza(NewYork())
