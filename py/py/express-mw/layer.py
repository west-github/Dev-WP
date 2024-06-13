from typing import Any, Callable
from .response import Response
from .request import Request
from .route import callable


class Layer:

    def __init__(self, func: callable) -> None:
        self.func = func

    def handle_request(
        self, req: Request, res: Response, next: Callable[[Any], None]
    ) -> None:

        try:
            self.func(req, res, next)
        except Exception as e:
            next(e)
