p = int(input())

mat = [[int(i) for i in input().split(' ')] for _ in range(p)]

for r,node in enumerate(mat):
    for i,child in enumerate(node):
        if i==r: continue
        for j,subchild in enumerate(mat[i]):
            if mat[r][j]!=0:
                print(f'{r+1} {i+1} {j+1}')


