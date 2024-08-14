# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        if head is None:
            return True
        if head.next is None:
            return True
        p1 = head
        p2 = head.next
        if p2.next is None:
            return p1.val == p2.val
        p2 = p2.next
        n = 0
        while p2 is not None:
            p1 = p1.next
            p2 = p2.next
            n += 1
            if p2 is None:
                break
            p2 = p2.next
            n += 1
        prev = None
        p3 = p1 if n&1 else p1.next
        while p3 is not None:
            next = p3.next
            p3.next = prev
            prev = p3
            p3 = next
        node1 = head
        node2 = prev
        i = 0
        while node1.val == node2.val:
            node1 = node1.next
            node2 = node2.next
            i += 1
            if i > n//2:
                return True
        return False
