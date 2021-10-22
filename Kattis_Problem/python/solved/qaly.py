# Solved

def main():
    N = int(input())
    print(sum((lambda q,y: float(q)*float(y))(*input().split()) for _ in range(N)))

main()