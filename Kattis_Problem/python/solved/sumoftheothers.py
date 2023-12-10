try:
    while 1:
        l = list(map(int,input().split()))
        s = sum(l)
        for x in l:
            if x == s-x:
                print(x)
                break
except:
    pass
