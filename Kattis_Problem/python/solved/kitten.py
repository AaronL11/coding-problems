# Solved

i = input()
K = int(i)
tree = dict()
heads = set()
leaves = set()
i = input()
while 1:
    if i.strip() == '-1': break
    branch = i.split()
    head, branches = int(branch[0]), [int(l) for l in branch[1:]]
    del branch
    heads.add(head)
    leaves |= set(branches)
    tree[head] = branches
    i = input()

root = {i for i in heads if i not in leaves}

def dfs(find, root, tree):
    stack = [root]
    found = False
    if root == find:
        return (True, stack)
    if root not in tree:
        return (False, stack)
    for k in tree[root]:
        found,s = dfs(find,k,tree)
        if found:
            stack += s
            return (True, stack)
    return (False, stack)

_,r = dfs(K,list(root)[0], tree)
print(*reversed(r))

