# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

def treeToList(node: Optional[TreeNode])-> List[int]:
    if (node == None): 
        return []
    return treeToList(node.left) + [node.val] + treeToList(node.right)

def getMaxIndex(nums: List[int]) ->(int, int):
    max = 0
    maxIndex =0
    for i in range(0, len(nums)):
        if nums[i] > max:
            max=nums[i]
            maxIndex = i
            
    return (maxIndex, max)
    
def buildMaxTree(nums: List[int])->Optional[TreeNode]:
    l = len(nums)
    
    if l == 0:
        return None
    
    if l == 1:
        return TreeNode(nums[0])
    
    (maxIndex, max) = getMaxIndex(nums)
    
    node = TreeNode(max)
    
    node.left = buildMaxTree(nums[0:maxIndex])
    node.right = buildMaxTree(nums[maxIndex+1:l])
    
    return node
    
class Solution:
    def insertIntoMaxTree(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        list = treeToList(root)
        list.append(val)
        
        return buildMaxTree(list)