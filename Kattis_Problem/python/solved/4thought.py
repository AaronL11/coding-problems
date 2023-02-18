def solve(CHARS,n):
    for a in CHARS:
        for b in CHARS:
            for c in CHARS:
                string = f'4 {a} 4 {b} 4 {c} 4'
                if eval(string) == n:
                    print(f'4 {"/" if a=="//" else a} 4 {"/" if b=="//" else b} 4 {"/" if c=="//" else c} 4 = {n}')
                    return True
    return False
     
m = int(input())
CHARS = ['+','-','*','//']
for i in range(m):
    n = int(input())
    if not solve(CHARS, n):
        print("no solution")

