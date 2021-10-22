coord_lst = []

for _ in range(0,3):
    coord_lst.append((int(x) for x in input().split(' ')))

appearedx = {}
appearedy = {}

for x,y in coord_lst:
    if x in appearedx:
        appearedx[x] += 1
    else:
        appearedx.update({x: 1})
    if y in appearedy:
        appearedy[y] += 1
    else:
        appearedy.update({y: 1})
x = 0
y = 0
for k in appearedx:
    if appearedx[k] < 2:
        x = k

for k in appearedy:
    if appearedy[k] < 2:
        y = k
print(f"{x} {y}")