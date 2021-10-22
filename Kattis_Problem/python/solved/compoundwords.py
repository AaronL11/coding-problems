# Solved

from itertools import combinations, permutations
s=[]
while True:
    try:
        s += input().split()
    except Exception:
        break

print('\n'.join(sorted({f'{x}{y}' for x,y in permutations(s,2)})))

