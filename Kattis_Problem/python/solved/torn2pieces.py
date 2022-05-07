N = int(input())

graph = {}

for _ in range(N):
    stations = input().split()
    graph[stations[0]] = set(stations[1:])
    for station in stations[1:]:
        try:
            graph[station].add(stations[0])
        except KeyError:
            graph[station] = {stations[0]}

source,destination = input().split()
stack = [source]
pathMap = {}
path = []

visited = set()

while stack:
    node = stack.pop()
    if node in visited:
        continue
    visited.add(node)
    if node == destination:
        break
    try:

        for next in graph[node]:
            if next in visited:
                continue
            stack.append(next)
            pathMap[next] = node
    except KeyError:
        continue

try:
    path = []
    curr = destination
    while curr != source:
        path.append(curr)
        curr = pathMap[curr]
    path.append(source)
    print(' '.join(reversed(path)))
except KeyError:
    print('no route found')



