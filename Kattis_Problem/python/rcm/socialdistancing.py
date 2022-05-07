S,N = map(int,input().split())
seats = [True for _ in range(S)]
for i in map(int,input().split()):
    i = i-1
    seats[i-1] = False
    seats[i] = False
    if i+1 < S:
        seats[i+1] = False
    else:
        seats[0] = False
sum = 0
p = seats[-1]
n = seats[1]
for i in range(S):
    s = seats[i]
    if s and s != p:
        seats[i-1] = False
        seats[i] = False
        if i+1 < S:
            seats[i+1] = False
        sum += 1
    p = s
    n = seats[(i+1)%S]
print(sum)

