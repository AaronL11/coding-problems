line1 = [int(x) for x in input().split(' ')]

socks = line1.pop(0)
cap = line1.pop(0)
max_diff = line1.pop(0)
del line1

sock_lst = [int(x) for x in input().split(' ')]
sock_lst.sort()

wash_mchn = 1
sks_wsh_mn = 1
low_mchn = sock_lst.pop(0)

for sock in sock_lst:
    if sks_wsh_mn == cap:
        wash_mchn += 1
        sks_wsh_mn = 0
        low_mchn = sock
    if sock - low_mchn > max_diff:
        wash_mchn += 1
        sks_wsh_mn = 0
        low_mchn = sock
    sks_wsh_mn += 1

print(wash_mchn)