def f(n):
    if n==1:
        return 2
    elif n<10:
        return 3
    elif n<1000000000:
        return 4
    else:
        return 5


s = input()
while s != "END":
    print(1 if s=="1" else f(len(s)))
    s = input()

