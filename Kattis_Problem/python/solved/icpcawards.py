stop = 0
winners = {}
for i in range(int(input())):
    if stop==12:
        break
    k,v = input().split()
    if k not in winners:
        winners[k]=v
        stop += 1

for k,v in winners.items():
    print(k,v)

