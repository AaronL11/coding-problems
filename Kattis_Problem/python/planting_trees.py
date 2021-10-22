trees = int(input())

tree_lst = [int(x) for x in input().split(' ')]

tree_lst.sort(reverse=True)

days = tree_lst.pop(0)
days_passed = 1
for tree in tree_lst:
    if tree + days_passed > days:
        days = tree + days_passed
    days_passed += 1

print(days + 2)

