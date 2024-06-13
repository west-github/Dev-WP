from typing import Any, Callable
from .request import Request
from .response import Response
from .route import Route


def use_handler(req: Request, next: Callable[[Any], None]) -> None:
    print(f"Got a request with data: {req}")
    next(None)
    ...


def clear_res(req: Request, next: Callable[[Any], None]) -> None:
    next(None)


def res_handler(req: Request, res: Response, next: Callable[[Any], None]) -> None:
    res.dat = "The soldiers are coming"


(
    Route()
    .use(use_handler)
    .use(use_handler)
    .use(use_handler)
    .use(clear_res)
    .use(res_handler)
    .use(use_handler)
    .use(use_handler)
    .use(use_handler)
    .dispatch(Request("Some Data"), Response("Empty Mutable Response"))
)
