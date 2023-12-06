n,k = map(int, input().split())
l = input().split()
i = 0
ans = 0
stack = []
while i < len(l):
    x = l[i]
    try:
        j = int(x)
        if j >= 0:
            ans = (ans + j) % n
        else:
            ans = (ans + n-j) % n
        stack.append(-j)
    except ValueError:
        i += 1
        m = int(l[i])
        for _ in range(m):
            if len(stack) != 0:
                j = stack.pop()
                if j >= 0:
                    ans = (ans + j) % n
                else:
                    ans = (ans + n-j) % n
            else:
                break
    i += 1

print(ans)
