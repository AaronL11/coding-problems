# UNSolved TOO SLOW

import pdb


def median(l: list) -> int:
    n = len(l)
    if n%2==0:
        k = n//2
        return (l[k]+l[k-1])//2
    else:
        return l[n//2]

def main():
    T = int(input())
    for _ in range(T):
        N = int(input())
        A = [int(i) for i in input().split()]
        result = 0
        for i in range(len(A)):
            l = A[:i+1]
            l.sort()
            result += median(l)
        print(result)

main()
