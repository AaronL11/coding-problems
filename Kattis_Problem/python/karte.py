# UnSolved

suits = {
    'P': 13,
    'K': 13,
    'H': 13,
    'T': 13,
}

string = input()

i=0
output = ''
seen = set()
stop=False
while i<len(string):
    s = string[i]
    i+=1
    n = int(string[i:i+2])
    if (s,n) in seen:
        stop=True
        print('GRESKA')
        break
    seen.add((s,n))
    suits[s] -= n
    i+=2
if not stop:
    print(*suits.values())
