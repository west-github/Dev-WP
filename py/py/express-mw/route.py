from typing import Any, Callable, Self
from .layer import Layer
from .request import Request
from .response import Response


class Route:

    def __init__(self) -> None:
        self.stack: list[Layer] = []

    def dispatch(self, req: Request, res: Response) -> None:
        idx = 0

        if not self.stack:
            # call done
            ...

        # error not used we can use it later
        def next(err: Any | None = None) -> None:
            nonlocal idx
            try:
                layer = self.stack[idx]
                idx += 1

                layer.handle_request(req, res, next)
            except IndexError:
                print("Hit the snag")
                # call done
                ...

        next()

    def use(
        self, func: Callable[[Request, Response, Callable[[Any], None]], None]
    ) -> Self:
        self.stack.append(Layer(func))
        return self


def new_router() -> Route:
    return Route()
