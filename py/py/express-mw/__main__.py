from typing import Any, Callable
from .request import Request
from .response import Response
from .route import new_router


def use_handler(req: Request, res: Response, next: Callable[[Any], None]) -> None:
    print(f"Got a request with data: {req}")

    next(None)
    ...


def clear_res(req: Request, res: Response, next: Callable[[Any], None]) -> None:
    res.dat = ""
    next(None)


def res_handler(req: Request, res: Response, next: Callable[[Any], None]) -> None:
    res.dat = "The soldiers are coming"


router = (
    new_router()
    .use(use_handler)
    .use(use_handler)
    .use(use_handler)
    .use(clear_res)
    .use(res_handler)
    .use(use_handler)
    .use(use_handler)
    .use(use_handler)
)

res = Response("Empty Mutable Response")
router.dispatch(Request("Some Data"), res)


print(res)
