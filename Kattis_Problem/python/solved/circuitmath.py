from string import ascii_uppercase

n = int(input())
l = list(map(lambda x: x=='T', input().split()))
stack = []
ans = False
for c in input().split():
    if c in ascii_uppercase:
        i = ord(c) - 65
        stack.append(l[i])
    elif c == '*':
        x = stack.pop()
        y = stack.pop()
        stack.append(x and y)
    elif c == '+':
        x = stack.pop()
        y = stack.pop()
        stack.append(x or y)
    else:
        x = stack.pop()
        stack.append(not x)
print('T' if stack.pop() else 'F')

