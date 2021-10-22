pillars = int(input())

bomb_num = 0
found = False
while True:
    if pillars <= 3:
        bomb_num += 1
        break
    else:
        bomb_num += 1
        pillars -= 3
        if pillars <= 3:
            bomb_num += 1
            break
        else:
            pillars += 2
print(bomb_num)