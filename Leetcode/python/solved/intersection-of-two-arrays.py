class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        def num2dict(nums: List[int]) -> dict[int,int]:
            map = {}
            for num in nums:
                if num in map:
                    map[num] += 1
                else:
                    map[num] = 1
            return map
        map1 = num2dict(nums1)
        map2 = num2dict(nums2)
        set1 = set(map1.keys())
        set2 = set(map2.keys())
        intr = set1&set2
        return [n
            for n in intr
            for _ in range(min(map1[n], map2[n]))
        ]
