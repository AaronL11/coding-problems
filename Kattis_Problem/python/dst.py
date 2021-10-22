# Solved

test = int(input())

for i in range(test):
    R,D,H,M = input().split(' ')

    D,H,M = int(D),int(H),int(M)

    nh = D // 60
    nmin = D % 60

    if R == 'F':

        tmin = (M + nmin) % 60

        ah = (M + nmin) // 60

        th = (H + nh + ah) % 24

        print(f'{th} {tmin}')
    if R == 'B':
        
        tmin = (M - nmin) % 60

        ah = (M - nmin) // 60

        th = (H - nh + ah) % 24

        print(f'{abs(th)} {abs(tmin)}')