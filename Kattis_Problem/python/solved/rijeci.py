# Solved

from math import sqrt

def fib(n):
    s5 = sqrt(5)
    phi = (1 + s5)/2
    Phi = (1 - s5)/2
    return (int((phi**(n-1)-Phi**(n-1))/s5),int((phi**n-Phi**n)/s5))

def main():
    print(*fib(int(input())))

main()
