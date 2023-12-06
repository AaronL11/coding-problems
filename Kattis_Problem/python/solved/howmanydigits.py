from math import log,floor,pi,e

while 1:
    try:
        n = int(input())
        print(
            1 if n==0 or n==1
            else floor(log(2*pi*n,10)/2+n*(log(n,10)-log(e,10)))+1
        )
    except:
        break
