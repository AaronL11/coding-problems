# Solved

def main():
    string = input()
    stack = []
    inside = False
    temp = 0
    for i,char in enumerate(string):
        if char in {':',';'}:
            inside = True
            temp = i
        elif char == '-' and inside:
            continue
        elif char == ')' and inside:
            stack.append(temp)
            inside = False
            temp = 0
        else:
            inside = False
            temp = 0

    for i in stack:
        print(i)

main()