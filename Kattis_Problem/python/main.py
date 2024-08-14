r,c = map(int,input().split())

Map = [[[int(i),0] for i in input().strip()] for _ in range(r)]

k = 0
for i in range(r):
    for j in range(c):
        if Map[i][j][1] != 0:
            continue
        p = Map[i][j][0]
        stack = [(i,j)]
        k += 1
        while len(stack) != 0:
            x,y = stack.pop()
            print(x,y)
            print(stack)
            Map[x][y][1] = k
            for dx,dy in [(-1,0),(1,0),(0,-1),(0,1)]:
                if 0 <= x+dx < r and 0 <= y+dy < c:
                    print(x+dx,y+dy)
                    print(Map[x+dx][y+dy])
                    if Map[x+dx][y+dy][0] == p and Map[x+dx][y+dy][1] != 0:
                        stack.append((x+dx,y+dy))

print('\n'.join(''.join(str(p[1]) for p in row) for row in Map ))

n = int(input())
for _ in range(n):
    r1,c1,r2,c2 = [int(i)-1 for i in input().split()]
    p1,k1 = Map[r1][c1]
    p2,k2 = Map[r2][c2]
    if (r1,c1) == (r2,c2):
        print('binary' if p1 == 0 else 'decimal')
    elif p1 != p2:
        print('neither')
    elif k1 == k2:
        print('binary' if p1 == 0 else 'decimal')
    else:
        print('neither')