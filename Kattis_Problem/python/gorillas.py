test_cases = input()

a,C = input().split(' ')

obstacles = int(input())

y = A(x-a)**2 + B(x-a) + C

dy = 2A(x-a) + B

for _ in range(obstacles):
    coords = input()