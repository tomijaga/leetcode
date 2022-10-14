class Solution:
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        fast = head
        slow = ListNode(-1)
        slow.next = head
        
        len = 0
        
        while fast:
            fast = fast.next
            len += 1
            
            if fast:
                fast = fast.next
                slow = slow.next
                
                len+=1
            
        if len == 1:
            return None
        
        mid = floor(len / 2)
        
        if slow.next:
            slow.next = slow.next.next
        
        return head