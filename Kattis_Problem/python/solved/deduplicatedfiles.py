from functools import reduce
n = int(input())
while n != 0:
    v = []
    hash = {}
    cnt = 0
    for _ in range(n):
        l = input()
        v.append(l)
        h = reduce(lambda x,y: x^y,map(ord,l))
        if h in hash:
            if all(l!=x for x in hash[h]):
                cnt += 1
            hash[h].append(l)
        else:
            hash[h] = [l]
            cnt += 1
    cnt2 = 0
    for x in hash.values():
        for i in range(len(x)):
            for j in range(i+1,len(x)):
                if x[i] != x[j]:
                    cnt2 += 1
    print(cnt,cnt2)
    n = int(input())
