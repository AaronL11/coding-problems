
i,j = tuple(
    complex(*[float(i) for i in input().split()])
    for _ in range(2)
)
z = i/j
print(f"{z.real} {z.imag}")


