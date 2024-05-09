from dataclasses import dataclass
from typing import Callable


@dataclass(kw_only=True)
class Req:
    data: str


@dataclass(kw_only=True)
class MutableResponse:
    data: str | None = None


type Handler = Callable[[Req, MutableResponse, Callable[[], None]], None]


def sequence(handlers: list[Handler]) -> Handler:

    def handler(req: Req, res: MutableResponse, next: Callable[[], None]) -> None:
        def _handler(idx: int) -> None:
            while idx < len(handlers):
                return handlers[idx](req, res, lambda: _handler(idx + 1))
            return next()

        return _handler(0)

    return handler


def middleware_one(req: Req, res: MutableResponse, next: Callable[[], None]) -> None:
    print(f"Got this value in middleware one: {req} - {res}")

    res.data = "Middleware one mutate response data"

    # If you didn't call next the next middleware in chain won't be called and response will be return  if you
    next()


def middleware_two(req: Req, res: MutableResponse, next: Callable[[], None]) -> None:
    print(f"Got this value in middleware two: {req} - {res}")

    next()


res = sequence([middleware_one, middleware_two])

res(
    Req(data="Some data in bytes"),
    MutableResponse(),
    lambda: print("This is the handler function that contain your application logic"),
)
