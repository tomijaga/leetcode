from copy import deepcopy

class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        prev = None
        trav = head
        
        n = 0
        while trav != None:
            next = trav.next
            
            new_node = ListNode(trav.val)
            new_node.next = prev
            prev = new_node
            
            trav = next
            n+=1
        
        
        for i in range(0, floor(n/2)):
            if prev.val != head.val:
                return False
            
            prev = prev.next
            head = head.next
        
        return True
        