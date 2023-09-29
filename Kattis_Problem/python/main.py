C = int(input())
for _ in range(C):
    arr = list(map(int,input().split()))
    av = sum(arr)/len(arr)
    print(len(arr))
    print(sum(arr))
    print(av)
    print(f'{sum(x for x in arr if x >= av) / len(arr):.3f}%')