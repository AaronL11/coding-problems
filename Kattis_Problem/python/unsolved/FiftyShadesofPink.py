import re

patterns = ["pink", "rose"]

n = int(input())

cache = [input().lower() for _ in range(0,n)]

num = 0

for label in cache:
    for pattern in patterns:
        if re.search(pattern, label):
            num += 1
            label = label.replace(label, '')
if num > 0:
    print(num)
else:
    print("I must watch Star Wars with my daughter")