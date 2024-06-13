import asyncio
from dataclasses import dataclass, field
import json
from typing import Self


@dataclass
class User:
    #
    name: str = field(default_factory=lambda: "West is born blessed")
    email: str = field(kw_only=True)
    age: int = field(default=20)

    def __enter__(self) -> Self:
        print(f"Entering the field with {self.name} and age {self.age}")
        return self

    def __exit__(*exc_info) -> None:
        print(f"Leaving the with field {exc_info}")

    # def __aenter__(self) -> None:
    #     print(f"Entering the async field with {self.name} and age {self.age}")

    # def __aexit__(*exc_info) -> None:
    #     print("Leaving the async field")


if __name__ == "__main__":
    user, _user = User("West", email="West@west.com"), User(email="West@west.com")

    user_e = [
        {
            "name": "East",
            "email": "east@east.com",
            "age": 18,
        },
        {
            "name": "East",
            "email": "east@east.com",
            "age": 18,
        },
        {
            "name": "East",
            "email": "east@east.com",
            "age": 18,
        },
        {
            "name": "East",
            "email": "east@east.com",
            "age": 18,
        },
    ]

    users = [User(**user) for user in user_e]

    with User("East", email="East@east.com", age=30) as user:
        for _ in range(0, 10, 2):
            # print(f"{user.name} {user.email} {user.age}")
            print(user)

    # Didn't compile for some reason will check back later
    # async def coroutine() -> None:

    #     async with User("East", email="East@east.com", age=30) as user:
    #         for _ in range(0, 10, 2):
    #             # print(f"{user.name} {user.email} {user.age}")
    #             print(user)

    # with asyncio.Runner() as r:
    #     r.run(context=coroutine())

    with open("index.txt", mode="a") as f:
        print((user), (_user), (user_e), sep="\n", file=f)
