v = set(["aa","ee","ii","oo","uu","yy"])
n = int(input())
g = {}
while n!=0:
    for _ in range(n):
        x = input()
        s = len("".join("." if x[i-1]+x[i] in v else "" for i in range(1,len(x))))
        sum = 0
        for i in range(1,len(x),2):
            print(i,x[i-1],x[i])
            if x[i-1]+x[i] in v:
                sum += 1
        g[sum]=x
    print(g[max(g.keys())])
    n = int(input())
exit()
