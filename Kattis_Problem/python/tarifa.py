# Solved

def main():
    MB = int(input())
    N = int(input())
    sum = 0
    for _ in range(N):
        i = int(input())
        sum += MB-i
    sum += MB
    print(sum)

main()