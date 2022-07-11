"""
# Definition for a Node.
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.parent = None
"""

class Solution:
    def inorderSuccessor(self, node: 'Node') -> 'Optional[Node]':
        if node is None:
            return None
        
        successor = [Node(sys.maxsize)]
        trav(node, node.val, successor)
        
        # print(successor)
        
        if successor[0].val == sys.maxsize:
            return None
        else:
            return successor[0]
        
        
def trav(node: 'Node', val: int,  successor: List['None']): 

    if node.parent is not None:
        trav(node.parent, val, successor)
    else:
        if node.left is not None:
            node.left.parent = None
            trav(node.left, val, successor)
            
        if node.right is not None:
            node.right.parent = None
            trav(node.right, val, successor)
    
    # print(node.val)
    if node.val > val:
        if node.val < successor[0].val:
            successor[0] = node
        
        