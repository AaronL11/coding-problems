N,M = map(int,input().split())
list = []
for c in range(1,N+1):
    line = input().split()
    sum = 0
    for i,w in zip(range(1,M+1),line):
        if i % 15 == 0:
            if w == "fizzbuzz":
                sum += 1
        elif i % 3 == 0:
            if w == "fizz":
                sum += 1
        elif i % 5 == 0:
            if w == "buzz":
                sum += 1
        elif str(i) == w:
            sum += 1
    list.append((sum,-c))
list.sort()
print(-list[-1][1])

