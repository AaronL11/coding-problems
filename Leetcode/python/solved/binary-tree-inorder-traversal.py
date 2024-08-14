# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        def traverseR(node: Optional[TreeNode], stack):
            if node is None:
                return
            traverseR(node.left, stack)
            stack.append(node.val)
            traverseR(node.right, stack)
        stack = []
        traverseR(root, stack)
        return stack

