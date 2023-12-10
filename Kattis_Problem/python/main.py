from math import ceil

def V(n, m, a, cache={}):
    if (n,m) in cache:
        return 0
    if n <= a and m <= a:
        cache[(n,m)] = 0
        return 1
    elif n <= a:
        cache[(n,m)] = 0
        return V(n,m-a,a) + 1
    elif m <= a:
        cache[(n,m)] = 0
        return V(n-a,m,a) + 1
    else:
        cache[(n,m)] = 0
        return V(n-a,m,a) + V(n,m-a,a) + 1

n,m,a = map(int,input().split())
print(V(n,m,a))
print(ceil(n/a) * ceil(m/a))