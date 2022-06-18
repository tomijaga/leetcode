# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        
        if head == None:
            return None
        
        trav = head
        dup = False
        
        while trav.next and trav.next.val == trav.val:
            trav = trav.next
            dup = True
            
        if dup:
            trav = self.deleteDuplicates(trav.next)
        else:
            trav.next = self.deleteDuplicates(trav.next)
        
        return trav
            