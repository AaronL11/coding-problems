# unsolved

def hadamard(n):
    if n ==1:
        return [n]
    else:
        return [
            [*hadamard(n-1),  *hadamard(n-1)],
            [*hadamard(n-1), *map(lambda x: -x, hadamard(n-1))]
            ]

H = hadamard

print(H(1))
print(H(2))
print(H(3))


