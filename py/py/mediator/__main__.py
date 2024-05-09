from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any


class Mediator(ABC):

    @abstractmethod
    def notify(self, ev: str) -> None: ...


class Component(ABC):

    mediator: Mediator

    def __init__(self, mediator: Mediator = None) -> None:
        self.mediator = mediator

    def set_mediator(self, mediator: Mediator) -> None:
        self.mediator = mediator


class One(Component):

    def called(self) -> None:
        print("One is called we can do something here")

        self.mediator.notify("ONE")

    def reaction(self) -> None:
        print("Reaction of one from mediator")


class Two(Component):

    def called(self) -> None:
        print("Two is called we can do something here")

        self.mediator.notify("TWO")

    def reaction(self) -> None:
        print("Reaction of two from mediator")


class Client(Mediator):

    one: One
    two: Two

    def __init__(self) -> None:
        self.one = One(self)
        self.two = Two(self)

    @staticmethod
    def create() -> list[One, Two]:
        client = Client()
        return [client.one, client.two]

    def notify(self, ev: str) -> None:

        if ev == "ONE":
            print("React on One")
            one.reaction()

        if ev == "TWO":
            print("React on Two")
            two.reaction()


@dataclass(kw_only=True)
class BaseEmployee(ABC):
    name: str
    age: int


class Hr(BaseEmployee):
    def accept(self, visitor: Any) -> int:
        return visitor.hr_salary(self)


class Employee(BaseEmployee):
    def accept(self, visitor: Any) -> int:
        return visitor.emp_salary(self)


class IVisitor(ABC):

    @abstractmethod
    def hr_salary(self, hr: Hr) -> int: ...

    @abstractmethod
    def emp_salary(self, emp: Employee) -> int: ...


class Konga(IVisitor):

    def hr_salary(self, hr: Hr) -> int:
        print(f"We Pay {hr.name} Hr 50 bucks with age: {hr.age}")

        return 30

    def emp_salary(self, emp: Employee) -> int:
        print(f"We pay {emp.name} Employee 30 Bucks with age: {emp.age}")

        return 40


"""
    Factory Pattern
    Abstract Factory
    Builder
    Singleton
    Prototype
    Adapter
    Bridge
    Composite
    Decorator
    Facade
    Flyweight
    Proxy
    
    Chain Of Responsibility
    Observer
    Memento
    Iterator
    State
    Strategy
    Mediator
    Command
    Template
    Visitor
    
    
    
"""
if __name__ == "__main__":
    [one, two] = Client.create()

    one.called()
    two.called()

    konga = Konga()

    employees = [
        Hr(name="West", age=30),
        Hr(name="East", age=20),
        Employee(name="North", age=15),
        Hr(name="West", age=30),
        Hr(name="East", age=20),
        Employee(name="North", age=15),
        Hr(name="West", age=30),
        Hr(name="East", age=20),
        Employee(name="North", age=15),
        Hr(name="West", age=30),
        Hr(name="East", age=20),
        Employee(name="North", age=15),
        Hr(name="West", age=30),
        Hr(name="East", age=20),
        Employee(name="North", age=15),
    ]

    total = sum([value.accept(konga) for value in employees])
    print(f"The total amount is {total}")
