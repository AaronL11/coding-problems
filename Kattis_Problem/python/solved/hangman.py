# Solved

def main():
    word = set(input())
    win = len(word)
    guess = input()
    total = 0
    guessed = False
    i = 0
    while not guessed:
        if guess[i] in word:
            win -= 1
        else:
            total += 1
        if win == 0:
            print('WIN')
            break
        elif total == 10:
            print('LOSE')
            break
        i+=1

main()
