# UnSolved TOO SLOW

def main():
    cities = {}

    n = int(input())
    for _ in range(n):
        s,y = input().split()
        y = int(y)
        if s in cities:
            cities[s].append(y)
            cities[s].sort()
        else:
            cities[s] = [y]
    q = int(input())
    stack = []
    for _ in range(q):
        s,k = input().split()
        k = int(k)
        stack.append(cities[s][k-1])
    for i in stack:
        print(i)

main()