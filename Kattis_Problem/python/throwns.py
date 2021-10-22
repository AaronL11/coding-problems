# Solved

s, t = input().split(' ')

s, t = int(s), int(t)

ins = input().split(' ')

stack = []

spot = 0
watch = False

for e in ins:
    if e == 'undo':
        watch = True
    elif watch:
        e = int(e)
        for _ in range(e):
            spot -= stack.pop() % s
        watch = False
    else:
        e = int(e)
        stack.append(e)
        spot += e % s
    spot %= s
        
print(spot)

