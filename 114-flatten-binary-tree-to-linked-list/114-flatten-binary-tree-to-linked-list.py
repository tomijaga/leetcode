# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
def split(root: Optional[TreeNode]) -> None:
    if root == None:
        return
    
    left = root.left
    root.left = None
    
    split(left)
    
    split(root.right)
    
    if left:
        right = root.right
        root.right = left
        
        while (left.right):
            left = left.right
        else:
            left.right = right
    
class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        split(root)
        
