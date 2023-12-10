ds,ys = list(map(int, input().split()))
dm,ym = list(map(int, input().split()))

cnt = 0
while ds != dm:
    ds = (ds + 1) % ys
    dm = (dm + 1) % ym
    cnt += 1
print(cnt)
