# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
def split(root: Optional[TreeNode]) -> None:
    if root == None:
            return None
    
    left = root.left
    root.left = None
    
    split(left)
    
    right = root.right
    split(right)
    
    if left:
        root.right = left
        trav = left
        
        while (trav.right):
            trav = trav.right
        else:
            trav.right = right
    else:
        root.right = right
    
class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        split(root)
        
