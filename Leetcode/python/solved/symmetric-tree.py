# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        lstack = [root.left]
        rstack = [root.right]
        while len(lstack) > 0 and len(rstack) > 0:
            nodel = lstack.pop()
            noder = rstack.pop()
            if nodel is None and noder is None:
                continue
            if nodel is None or noder is None:
                return False
            if nodel.val == noder.val:
                lstack.append(nodel.left)
                lstack.append(nodel.right)
                rstack.append(noder.right)
                rstack.append(noder.left)
            else:
                return False
        return True
