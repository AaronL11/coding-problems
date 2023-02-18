n,K = map(int,input().split())
print(min(K + 1+ (n-1)%K, n-1))

