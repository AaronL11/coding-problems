case_nums = int(input())

for case in range(0, case_nums):
    input()
    kids = int(input())
    total = 0
    candies = 0
    for kid in range(0, kids):
        candies += int(input())
        total += 1
    if candies % total == 0:
        print("YES")
    else:
        print("NO")