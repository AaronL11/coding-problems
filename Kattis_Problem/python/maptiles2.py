# Solved

M = {
    0: (0,0),
    1: (1,0),
    2: (0,1),
    3: (1,1)
}

s = input()
zoom = len(s)
size = 2**zoom
n=zoom
X,Y = 0,0
for c in s:
    x,y = M[int(c)]
    x *= 2**(n-1)
    y *= 2**(n-1)
    X += x
    Y += y
    n-=1
print(zoom,X,Y)
