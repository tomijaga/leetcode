"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""

from collections import deque

class Solution:
    def connect(self, root: 'Node') -> 'Node':
        if root == None:
            return None
            
        q2 = deque([root])
        
        while len(q2) > 0:
            q = q2
            q2 = deque()
            
            while len(q) > 0:
                item = q.popleft()
                # print(item.val)
                    
                if len(q) > 0:
                    item.next = q[0]
                else:
                    item.next = None
                
                if not (item.left == None):
                    q2.append(item.left)
                    
                if not (item.right == None):
                    q2.append(item.right)
        
            
        return root
            