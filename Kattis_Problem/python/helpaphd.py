# Solved

def main():
    N = int(input())

    for _ in range(N):
        i = input()
        print('skipped' if i=='P=NP' else eval(i))

main()
