# Solved

A,B,C,D,E = [int(i) for i in input().split(' ')]
g = int(input())
if g>=A: print('A')
elif A>g>=B: print('B')
elif B>g>=C: print('C')
elif C>g>=D: print('D')
elif D>g>=E: print('E')
elif E>g: print('F')


