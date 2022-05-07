MAX = 1_000_000_000_000_000
primes = [True for _ in range(MAX)]
for i in range(MAX):
    if primes[i]:
        for j in range(i*i):
            primes[j] = False
