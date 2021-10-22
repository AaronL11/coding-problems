# Solved

def main():
    d = {}
    i = input()
    while i != '-1':
        time,q,sub = i.split()
        if q not in d:
            d[q] = [(int(time),sub)]
        else:
            d[q].append((int(time),sub))
        i = input()

    sum = 0
    right = 0
    for l in d.values():
        penalty = 0
        for i in l:
            t,rw = i
            if rw == 'right':
                right += 1
                sum+=t+penalty
            else:
                penalty += 20

    print(right, sum)

main()
