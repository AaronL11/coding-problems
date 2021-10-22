import re

length, code, guess = [text for text in input().split()]
length = int(length)
r = 0
s = 0

cache_code = {}
cache_guess = {}

num = 0

while num < length-1:
    if code[num] == guess[num]:
        r += 1
        code = code.replace(code[num], '', 1)
        guess = guess.replace(guess[num], '', 1)
        length -= 1
    num += 1
for letter in code:
    if cache_code.get(letter) != None:
        cache_code[letter] += 1
    else:
        cache_code.update({letter: 1})
for letter in guess:
    if cache_guess.get(letter) != None:
        cache_guess[letter] += 1
    else:
        cache_guess.update({letter: 1})

code_keys = [key for key in cache_code.keys()]
guess_keys = [key for key in cache_guess.keys()]

code_keys.sort()
guess_keys.sort()

for num in range(0,len(code_keys)):
    if code_keys[num] == guess_keys[num]:
        s += 1

print(f"{r} {s}")