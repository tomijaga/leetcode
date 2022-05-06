# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

def verify(node:Optional[TreeNode], left_bound:Optional[int], right_bound: Optional[int]) -> bool:
    if node == None:
        return True
    val = node.val
    
    if left_bound !=None:
        if left_bound >= val:
            return False
        
    if right_bound != None:
        if right_bound <= val:
            return False
    
    left = verify(node.left, left_bound, val) 
    # print("l:{}, res:{}".format(node.left, left))
    right = verify(node.right, val, right_bound)
    # print("r:{}, res:{}".format(node.right, right))
        
    return left and right
    
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        return verify(root, None, None)