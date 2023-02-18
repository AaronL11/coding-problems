map = {}
a = input().strip()
while a != "***":
    try:
        map[a] += 1
    except:
        map[a] = 1
    a = input().strip()
mx = 0
curr = ""
runoff = False
for k,v in map.items():
    if v == mx:
        runoff = True
    elif v > mx:
        runoff = False
        mx = v
        curr = k
if runoff:
    print("Runoff!")
else:
    print(curr)
