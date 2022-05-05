# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
def split(root: Optional[TreeNode]) -> Optional[TreeNode]:
    if root ==None:
            return None
    
    left = root.left
    root.left = None
    
    left = split(left)
    right = split(root.right)
    
    if left:
        root.right = left
        trav = left
        
        while (trav.right):
            trav = trav.right
        else:
            trav.right = right
    else:
        root.right = right
    
    return root

class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        split(root)
        
