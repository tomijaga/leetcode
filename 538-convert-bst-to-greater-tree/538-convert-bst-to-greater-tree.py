# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


    
class Solution:
    sum = 0
    
    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def convert(root:Optional[TreeNode]):
            if root: 
                convert(root.right);
            
                self.sum += root.val
                root.val = self.sum

                convert(root.left)
        
        convert(root)
        
        return root
        
        