# """
# This is the ImmutableListNode's API interface.
# You should not implement it, or speculate about its implementation.
# """
# class ImmutableListNode:
#     def printValue(self) -> None: # print the value of this node.
#     def getNext(self) -> 'ImmutableListNode': # return the next node.

class Solution:
    def printLinkedListInReverse(self, head: 'ImmutableListNode') -> None:
        
        prev = None
        node = None
        while head:
            node = {
                "data": None,
                "next": None
            }
            
            node["data"] = head
            node["next"] = prev
            
            head = head.getNext()
            prev = node
        
        while node:
            node["data"].printValue()
            
            node = node["next"]