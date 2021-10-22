sequence = input()

pos = [1,0,0]

for move in sequence:
    if move == 'A':
        i = pos[1]
        pos[1] = pos[0]
        pos[0]  = i

    elif move == 'B':
        i = pos[2]
        pos[2] = pos[1]
        pos[1]  = i
    elif move == 'C':
        i = pos[2]
        pos[2] = pos[0]
        pos[0]  = i

for i,j in enumerate(pos):
    if j == 1:
        print(i + 1)