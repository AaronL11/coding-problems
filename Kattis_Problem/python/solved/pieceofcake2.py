n,h,v = [int(i) for i in input().split()]
q1,q2,q3,q4 = (n-v)*h,v*h,(n-h)*v,(n-h)*(n-v)
print(max(q1,q2,q3,q4)*4)
