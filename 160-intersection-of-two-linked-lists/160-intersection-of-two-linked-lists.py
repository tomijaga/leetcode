# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        if (headA == None or  headB == None):
            return None
        
        p1 = headA
        p2 = headB
        
        while (p1 and p2 and p1 != p2 ):
            p1 = p1.next
            p2 = p2.next
            
            if p1 == p2:
                return p1
            
            if p1 == None:
                p1 = headB
                
            if p2 == None:
                p2 = headA
                
            
        return p1