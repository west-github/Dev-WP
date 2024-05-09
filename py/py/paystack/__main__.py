import asyncio


async def task(args: str) -> None:
    for index in range(5):
        # await asyncio.sleep(1)

        print(f"Runner Main with args: {args} and index: {index}")


async def other(args: str) -> None:
    await task(args)


async def error() -> None:
    raise Exception("Error Occured we don't know the cause")


async def main(args: str):
    await task(args)


with asyncio.Runner(debug=True) as r:

    async def runner() -> None:
        [_main, _other, err] = await asyncio.gather(
            main("MAIN"), other("OTHER"), error(), return_exceptions=True
        )

        if isinstance(err, BaseException):
            print(f"We got err: {err}")

    r.run(runner())
