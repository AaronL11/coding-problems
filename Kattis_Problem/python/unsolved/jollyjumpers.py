# UnSolved

def main():
    try:
        while 1:
            string = input().split()
            if len(string) == 1:
                print('Jolly')
                continue
            n,string = int(string[0]),string[1:]
            jolly = True
            temp = n
            for i in range(n-1):
                x,y = int(string[i]),int(string[i+1])
                a = abs(x-y)
                if temp == a+1:
                    temp = a
                else:
                    jolly = False
                    break
            if jolly:
                print('Jolly')
            else:
                print('Not jolly')
    except: pass

main()

