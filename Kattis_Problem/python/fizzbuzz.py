# Solved

def fizz_buzz(**fb):
    def eval(i):
        s = ''.join(s for s,n in fb.items() if i%n==0)
        return s if s else i
    def gen(stop,start=1):
            i = start
            while i<=stop:
                yield eval(i)
                i += 1
    def closure(range=None, at=1):
        return gen(range,start=at) if range else eval(at)
    return closure

def main():
    X,Y,N = input().split()
    X,Y,N = int(X),int(Y),int(N)
    for i in fizz_buzz(Fizz=X,Buzz=Y)(range=N):
        print(i)

if __name__ == '__main__':
    main()