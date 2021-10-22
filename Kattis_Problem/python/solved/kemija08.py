# Solved

def main():
    vowels = {'a', 'e', 'i', 'o', 'u'}

    sentence = input()
    decoded = ''
    i = 0
    while i < len(sentence):
        if sentence[i] in vowels:
            i+=2
        decoded += sentence[i]
        i+=1

    print(decoded)

main()
