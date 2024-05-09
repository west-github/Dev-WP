from dataclasses import dataclass, field
import json


@dataclass
class User:
    #
    name: str = field(default_factory=lambda: "West is born blessed")
    email: str = field(kw_only=True)
    age: int = field(default=20)


if __name__ == "__main__":
    user, _user = User("West", email="West@west.com"), User(email="West@west.com")

    user_e = {
        "name": "East",
        "email": "east@east.com",
        "age": 18,
    }

    print(json.dumps(user_e))

    user_e = User(**user_e)  # type: ignore

    with open("index.txt", mode="a") as f:
        print((user), (_user), (user_e), sep="\n", file=f)
