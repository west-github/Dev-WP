import asyncio


async def task(args: str) -> None:
    for index in range(5):
        print(f"Runner Main with args: {args} and index: {index}")


async def other(args: str) -> None:
    await task(args)


async def main(args: str):
    await task(args)


with asyncio.Runner(debug=True) as r:

    async def runner() -> None:
        _ = await asyncio.gather(main("MAIN"), other("OTHER"), True)

    r.run(runner())
