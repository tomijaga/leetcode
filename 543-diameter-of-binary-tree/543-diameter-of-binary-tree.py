# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        
        if root == None:
            return 0
        
        _max = [0]
        dfs(root, _max)

        return _max[0]
    
def dfs(root: Optional[TreeNode], _max: List[int]) -> int:
    if root == None:
            return 0
    
    left = dfs(root.left, _max) 
    right = dfs(root.right,_max) 
    sum = left + right
    
    print(left, right, root.val)
    
    if sum > _max[0]:
        _max[0] = sum
        
    return max(left, right) + 1