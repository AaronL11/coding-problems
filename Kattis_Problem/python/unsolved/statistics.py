
case_num = 0
while True:
    try:
        case_num += 1

        buff = [int(x) for x in input().split(' ')]

        ln = buff.pop(0)

        mn = int(min(buff))
        mx =  int(max(buff))
        range = mx - mn
        print(f"Case {case_num}: {mn} {mx} {range}")
    except:
        break