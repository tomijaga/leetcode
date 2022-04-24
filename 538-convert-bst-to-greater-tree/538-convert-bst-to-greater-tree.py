# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

def convert(root:Optional[TreeNode], sum: int) -> int:
    if root.right:
        sum = convert(root.right, sum);
    
    sum += root.val
    root.val = sum
    
    if root.left:
        sum=convert(root.left, sum)
    
    return sum
    
class Solution:
    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        sum = 0
        if root:
            convert(root, sum)
        
        return root
        
        