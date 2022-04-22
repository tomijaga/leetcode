# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:   
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        l = len(nums)
        
        if l == 0:
            return None
        
        if l == 1:
            return TreeNode(nums[0])
        
        median_index = (int)(l/2)
        median = nums[median_index]
        node = TreeNode(median)
        
        node.left = self.sortedArrayToBST(nums[0:median_index])
        node.right = self.sortedArrayToBST(nums[median_index+1:l])
        
        return node
        