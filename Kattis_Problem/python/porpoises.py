# UnSolved

from functools import reduce

def fib(n):
    return reduce(lambda prev,_: (sum(prev), prev[0]), range(n-1), (1,0))[0]

cases = input()

for _ in range(int(cases)):
    case, yrs = input().split(' ')
    case, yrs = int(case), int(yrs)
    print(case, fib(yrs) % 10_000_000_000)

# print('\n'.join(f'{n}, {fib(n)}' for n in range(1,46)))
