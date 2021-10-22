# Solved

cases = input()

sum = 0

for _ in range(int(cases)):
    s = input()
    n = int(s[:-1])
    p = int(s[-1])
    sum += n**p

print(sum)
