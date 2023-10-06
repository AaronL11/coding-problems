while True:
    n = int(input())
    if n == 0: break
    cust = {}
    for _ in range(n):
        line = input().split()
        for ing in line[1:]:
            try:
                cust[ing].append(line[0])
            except:
                cust[ing] = [line[0]]
    for ing in sorted(cust):
        print(f"{ing} {' '.join(sorted(cust[ing]))}")
    print()

