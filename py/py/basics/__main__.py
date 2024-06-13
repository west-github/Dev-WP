import sys

global DEBUG


def pack_all(*args: int) -> None:
    for number in args:
        print(number)


def args_positional_optional_kw(pos_1: any, pos_2: any, /, pos_kw: any) -> None:
    [print(arg) for arg in [pos_1, pos_2, pos_kw]]


def args_kw_only(*, arg: any) -> None:
    print(arg)


def main() -> int:
    DEBUG = True
    try:

        pack_all(1, 2, 3, 4, 5, 6, 7, 8, 9)
        args_positional_optional_kw(1, 2, 4)
        # Let Say error happen in this line_num()
        raise Exception("Oops some bugs biting us hard")
        args_kw_only(arg="Some Value")

        return 0
    # Catch all Exception
    except BaseException as e:
        DEBUG and print(e)
        return 1


sys.exit(main())
