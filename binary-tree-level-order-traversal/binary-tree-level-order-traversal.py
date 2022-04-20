# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

def getLevel(stack: List[Optional[TreeNode]]):
    level = []
    nextStack = []
    while len(stack) > 0:
        node = stack.pop(0)
        if node.left:
            nextStack.append(node.left)
        if node.right:
            nextStack.append(node.right)
        
        
        level.append(node.val)
    return (level, nextStack)
    
class Solution:
    
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root == None:
            return []

        stack = [root]
        solution =[]
        while len(stack) > 0:
            (level, stack) = getLevel(stack)
            solution.append(level)
        
        return solution
        
            