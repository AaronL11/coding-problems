t = int(input())
for _ in range(t):
    p = input()
    n = int(input())
    l = input()[1:-1].split(',')
    i,j = 0,n-1
    r = False
    b = False
    for c in p:
        if c == 'R':
            r = not r
        else:
            if i > j:
                print('error')
                b = True
                break
            if r:
                j -= 1
            else:
                i += 1
    if not b:
        if r:
            print(f"[{','.join(reversed(l[i:j+1]))}]")
        else:
            print(f"[{','.join(l[i:j+1])}]")


