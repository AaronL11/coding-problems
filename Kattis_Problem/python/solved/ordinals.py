#!/bin/python3

def solution(n: int) -> str:
    return '{}' if n==0 else ','.join(solution(i) for i in range(n)).join('{}')

def main() -> None:
    print(solution(int(input())))

if __name__ == '__main__':
    main()



