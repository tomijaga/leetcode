from collections import deque

class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
        q = deque([root])
        right_nodes = []
        while len(q) > 0:
            size = len(q)
            for i in range(0, size):
                node = q.popleft()
                
                if i + 1 == size:
                    right_nodes.append(node.val)
                
                if node.left is not None:
                    q.append(node.left)
                    
                if node.right is not None:
                    q.append(node.right)                
        
        return right_nodes