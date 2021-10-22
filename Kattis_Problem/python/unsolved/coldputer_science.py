inputs = int(input())
temperatures = [int(x) for x in input().split(' ')]

num = 0

for temp in temperatures:
    if temp < 0:
        num += 1

print(num)
