input()
stack = []
b = True
for i,c in enumerate(input()):
    if c == ' ':
        ...
    elif len(stack) == 0:
        if c in '[({':
            stack.append(c)
        else:
            print(c, i)
            b = False
            break
    else:
        x = stack.pop()
        if c in '[({':
            stack.append(x)
            stack.append(c)
        elif x == '(' and c ==')':
            ...
        elif x == '[' and c == ']':
            ...
        elif x == '{' and c == '}':
            ...
        else:
            print(c, i)
            b = False
            break
if b:
    print('ok so far')
