before = input()
after = input()

while len(before) != 0 and len(after) != 0:
    if before[0] == after[0]:
        before = before[1:]
        after = after[1:]
    else:
        before = before[::-1]
        after = after[::-1]
        break
while len(before) != 0 and len(after) != 0:
    if before[0] == after[0]:
        before = before[1:]
        after = after[1:]
    else:
        break

print(len(after))