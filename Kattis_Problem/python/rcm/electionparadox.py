N = int(input())
wins,lose = N//2,N-N//2
regions = list(map(int,input().split()))
regions.sort()
votes = sum(regions[lose:])
votes += sum(n//2 for n in regions[:lose])
print(votes)


