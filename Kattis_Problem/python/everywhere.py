# Solved

T = int(input())

for _ in range(T):
    n = int(input())
    print(
        len({input() for _ in range(n)})
    )
