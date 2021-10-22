# Solved

def main():
    from math import sqrt
    N = int(input())
    for _ in range(N):
        msg = input()
        l = len(msg)
        size = int(sqrt(l))
        print(''.join(msg[i*size + j] for j in range(size-1,-1,-1) for i in range(size)))

main()