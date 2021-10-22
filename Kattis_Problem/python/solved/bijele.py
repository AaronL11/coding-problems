# Solved

pieces = [1,1,2,2,2,8]

print(' '.join(f'{pieces[i]-int(p)}' for i,p in enumerate(input().split())))