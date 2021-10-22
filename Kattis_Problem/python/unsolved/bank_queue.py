

N, T = input().split(' ')
N = int(N)
T = int(T)

ppl = {}
cost = []
time = []

for i in range(N):
    c,t = input().split(' ')
    c,t = int(c), int(t)
    cost.append(c)
    time.append((t,i))
    ppl.update({(t,i): c})
times = sorted(time)

serve = []
time_past = -1

current_max = 0
total = 0
while time_past <= T:
    for n,i in enumerate(time):
        if i == time_past:
            continue
        elif cost[n] > current_max:
            current_max = cost[n]
    total += current_max
    time_past += 1


for t in times:
    x = t[0]
    if x < T and x >= time_past:
        if ppl[t] > 
        serve.append(t)
        T -= x
    time_past += 1

income = sum(ppl[i] for i in serve)
print(income)