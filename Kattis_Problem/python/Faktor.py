articles, impact = (int(x) for x in input().split(' '))

impact -= 1

scientists = articles * impact

scientists += 1

print(scientists)