X, Y = [int(i) for i in input().split(' ')]
output = 'possible' if (4*X + 3*Y) % 2 == 0 else 'impossible'
print(output)