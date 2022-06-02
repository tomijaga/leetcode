# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def getDecimalValue(self, head: ListNode) -> int:
        n = 0;
        arr = []
        
        while(head):
            arr.append(head.val)
            head = head.next

        l = len(arr)
        for i in range(0, len(arr)):
            
            bit = arr[l - i -1]
            n  |= bit << i
            
        return n