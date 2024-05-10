from dataclasses import dataclass


@dataclass
class Slot:
    __slots__ = ["value", "content"]
    value: str

    content: str

    def __init__(self, value: str, content: str) -> None:
        self.value = value
        self.content = content


if __name__ == "__main__":
    slot = Slot("Value", "Content Value")

    print(slot)
