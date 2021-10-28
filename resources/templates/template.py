import sys
from typing import TextIO

def solve(scan: TextIO, out: TextIO) -> None:
    ...
    out.flush()


def main() -> None:
    scan = sys.stdin
    out = sys.stdout
    solve(scan, out)

main()
