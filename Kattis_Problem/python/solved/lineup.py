s = [input() for _ in range(int(input()))]
m = sorted(s)
if m==s:
    print("INCREASING")
elif [*reversed(m)]==s:
    print("DECREASING")
else:
    print("NEITHER")
