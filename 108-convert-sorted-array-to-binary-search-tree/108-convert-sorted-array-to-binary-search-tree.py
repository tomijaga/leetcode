# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution: 
    def bst(self, nums: List[int], start: int, end: int ) -> Optional[TreeNode]:
        l = end - start
        
        if end < start or l == 0:
            return None
        
        if l == 1:
            return TreeNode(nums[start])
        
        median_index = (int)(l/2) + start
        median = nums[median_index]
        node = TreeNode(median)
        
#         print("left: {start}, {end}".format(start = start, end = median_index))
#         print(nums[start:median_index])
        
#         print("right: {start}, {end}".format(start = median_index +1, end = end))
#         print(nums[median_index+1:l])
        node.left = self.bst(nums, start, median_index)
        node.right = self.bst(nums, median_index+1, end)
        
        return node
        
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        return self.bst(nums, 0, len(nums))
        