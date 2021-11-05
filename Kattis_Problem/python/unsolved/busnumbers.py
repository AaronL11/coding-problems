input()
s = [int(i) for i in input().split()]
if len(s)==1:
    print(s[0])
    exit()
s.sort()
l = []
m,k = 0,[]
import pdb
pdb.set_trace()
for i in s:
    k.append(i)
    if i==m+1:
        pass
    elif len(k)>2:
        l.append(f'{k[0]}-{k[-1]}')
        k.clear()
    else:
        for j in k:
            l.append(str(j))
        k.clear()
    m = i
print(" ".join(l))
