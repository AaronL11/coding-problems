# Solved

def main():
    from string import ascii_lowercase, digits, punctuation, whitespace
    letter = set(ascii_lowercase)
    N = int(input())
    for _ in range(N):
        sentence = set(input().lower())
        sentence -= set(digits)|set(punctuation)|set(whitespace)
        diff = letter-sentence
        if diff == set():
            print("pangram")
        else:
            print(f"missing {''.join(sorted(diff))}")

main()