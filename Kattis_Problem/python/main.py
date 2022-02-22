T = int(input())

def subset_sum(array: list,sum: int,set: set,memo):
    if sum in memo and memo[sum] != set:
        return (set, memo[sum])
    elif len(set) == 20:
        None
    else:
        for i in array:
            if i in set:
                continue
            set.add(i)
            sum += i;
            print(set)
            memo[sum] = set.copy()
            sets = subset_sum(array,sum,set,memo)
            if sets is not None:
                return sets
            sum -= i;
            set.remove(i)
        None

for i in range(T):
    print(f"Case #{i}")
    nums = [int(n) for n in input().split()]
    N,nums = nums[0],nums[1:]
    sets = subset_sum(nums,0,set(),{})
    if sets is None:
        print("Impossible")
    else:
        set1,set2 = sets
        print(' '.join(n for n in set1))
        print(' '.join(n for n in set2))

