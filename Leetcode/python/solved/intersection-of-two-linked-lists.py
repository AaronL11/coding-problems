# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        seen = set()
        link1 = headA
        link2 = headB
        while link1 is not None or link2 is not None:
            if link1 is not None:
                if link1 in seen:
                    return link1
                else:
                    seen.add(link1)
                link1 = link1.next
            if link2 is not None:
                if link2 in seen:
                    return link2
                else:
                    seen.add(link2)
                link2 = link2.next
