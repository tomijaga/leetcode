# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class BSTIterator(object):
    
    def __init__(self, root):
        """
        :type root: TreeNode
        """
        if root == None:
            return
        self.stack = []
        self.addAllToStack(root)

    def addAllToStack(self, root):
        while root:
            self.stack.append(root)
            root = root.left
        
    def next(self):
        """
        :rtype: int
        """
        return self.get_val()
            
    def get_val(self):
        val = None
        if len(self.stack) > 0:
            node = self.stack.pop()
            val = node.val
            if node.right:
                self.addAllToStack(node.right)
        
        return val

    def hasNext(self):
        """
        :rtype: bool
        """
        return len(self.stack) > 0