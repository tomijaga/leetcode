# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

def getMaxIndex(nums: List[int])-> int:
    max = 0
    maxi = 0
    for i in range(0, len(nums)):
        if nums[i]> max:
            max = nums[i]
            maxi = i
    return maxi
        
def maxTree(nums: List[int]) -> Optional[TreeNode]:
    l = len(nums);
    if l == 0:
        return None
    
    if l == 1:
        return TreeNode(nums[0])
    
    maxIndex = getMaxIndex(nums)
    max = nums[maxIndex]
    
    node = TreeNode(max);
    
    left = nums[0:maxIndex]
    right = nums[maxIndex+1:l]
    
    node.left = maxTree(left)
    node.right = maxTree(right)
    
    return node
    
class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> Optional[TreeNode]:
        return maxTree(nums)