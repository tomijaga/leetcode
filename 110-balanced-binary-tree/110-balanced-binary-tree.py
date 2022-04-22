# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    # returns -1 if the tree is not balanced
    def getBalancedHeight(self, node: Optional[TreeNode]) -> int:
        if node == None:
            return 0
        
        left = self.getBalancedHeight(node.left)
        right = self.getBalancedHeight(node.right)
        # print("{0}, {1} -> {2}".format(left, right, abs(right - left)))
        
        if left == -1 or right == -1:
            return -1
        
        
        if (abs(right - left) > 1 ):
            return -1
        
        return max(left, right) + 1
        
        
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        height = self.getBalancedHeight(root)
        if height == -1:
            return False
        
        return True