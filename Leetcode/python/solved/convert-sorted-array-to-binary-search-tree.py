# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        def recurse(nums: List[int]) -> Optional[TreeNode]:
            if len(nums) == 0:
                return None
            elif len(nums) == 1:
                return TreeNode(nums[0], None, None)
            elif len(nums) == 2:
                return TreeNode(
                    nums[0],
                    None,
                    TreeNode(
                        nums[1],
                        None,
                        None
                    )
                )
            else:
                l,r = 0,len(nums)-1
                mid = (l+r)//2
                root = TreeNode(nums[mid])
                root.left = recurse(nums[:mid])
                root.right = recurse(nums[mid+1:])
                return root
        return recurse(nums)
