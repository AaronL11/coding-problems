# # UnSolved

NUMBERS = {
    'A': {
        'dominant': 11,
        'not-dominant': 11
    },
    'K': {
        'dominant': 4,
        'not-dominant': 4
    },
    'Q': {
        'dominant': 3,
        'not-dominant': 3
    },
    'J': {
        'dominant': 20,
        'not-dominant':2
    },
    'T': {
        'dominant': 10,
        'not-dominant': 10
    },
    '9': {
        'dominant': 14,
        'not-dominant': 0
    },
    '8': {
        'dominant': 0,
        'not-dominant': 0
    },
    '7': {
        'dominant': 0,
        'not-dominant': 0
    }
}

def main():
    N,B =input().split()
    result = 0
    for _ in range(4*int(N)):
        n,s = input()
        result += NUMBERS[n]['dominant' if s==B else 'not-dominant']
    print(result)

main()
