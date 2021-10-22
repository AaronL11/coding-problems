# Solved

def main():
    name = input()
    fix = ''
    prev = ''
    for letter in name:
        fix += '' if letter==prev else letter
        prev = letter
    print(fix)

main()