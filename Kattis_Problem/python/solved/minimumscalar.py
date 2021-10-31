def main():
    T = int(input())
    for i in range(T):
        _ = input()
        a,b = [int(i) for i in input().split()],[int(i) for i in input().split()]
        a.sort()
        b.sort(reverse=True)
        print(f"Case #{i+1}: {sum(x*y for x,y in zip(a,b))}")

if __name__ == "__main__":
    main()

