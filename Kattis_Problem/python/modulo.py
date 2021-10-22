# Solved

buff = list()
distinct = list()

num = 0

for _ in range(0,10):
    buff.append(input())

for item in buff:
    remainder = int(item) % 42
    if remainder in distinct:
        num += 1
    else:
        distinct.append(remainder)

print(len(distinct))