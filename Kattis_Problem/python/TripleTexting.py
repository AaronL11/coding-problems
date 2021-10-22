word = input()

length = len(word)

word_len = length // 3

if word[:word_len] == word[word_len:word_len*2]:
    print(word[:word_len])
elif word[word_len:word_len*2] == word[word_len*2:]:
    print(word[word_len:word_len*2])
else:
    print(word[word_len*2:])