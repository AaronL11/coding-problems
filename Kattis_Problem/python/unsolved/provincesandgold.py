p = {
        'g': 3,
        's': 2,
        'c': 1
}
g,s,c = [int(i)*p[c] for c,i in zip('gsc',input().split())]




