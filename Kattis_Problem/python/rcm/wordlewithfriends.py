N,W = map(int,input().split())
Gs = ['']*5
Ys = set()
for _ in range(N):
    guess,result = input().split()

words = []
for _ in range(W):
    word = input()
    for i in range(5):
        if word[i] == Gs[i]:
            words.append(word)
            continue
    if len(set(word)-Ys) < len(word):
        ...

print('\n'.join(words))
