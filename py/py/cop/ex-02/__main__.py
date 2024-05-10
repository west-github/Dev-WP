from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Self


@dataclass
class User:
    name: str
    password: str


class Database:
    store: list[User]

    def __init__(self) -> None:
        self.store = []

    def add_user(self, user: User) -> None:
        self.store.append(user)

    def find_user(self, user: str) -> bool:
        return user in self.store


class BaseHandler(ABC):
    _next: Self | None = None

    def set_next(self, next: Self) -> Self:
        self._next = next
        return next

    @abstractmethod
    def handle(self, request: str) -> bool: ...

    def handle_next(self, request: str) -> bool:
        if not self._next:
            return False
        return self._next.handle(request)


class UserExists(BaseHandler):
    database: Database
    count: int

    def __init__(self, count: int, database: Database) -> None:
        self.count = count
        self.database = database

    def handle(self, request: str) -> bool:
        if not self.database.find_user(request):
            print("User Doesn't Exist")
            return False

        print(f"User is welcome with count: {self.count}")
        return super().handle_next(request)


class Service:
    handler: BaseHandler

    def __init__(self, handler: BaseHandler) -> None:
        self.handler = handler

    def login(self, request: str) -> bool:
        def on_login() -> bool:
            print("User is logged in")

        return self.handler.handle(request) and on_login()


if __name__ == "__main__":
    database = Database()
    database.add_user("West")

    handler = UserExists(1, database)
    handler.set_next(UserExists(2, database)).set_next(UserExists(3, database))

    service = Service(handler)

    if service.login("West"):
        print("Hey Welcome")
