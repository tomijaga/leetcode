"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        if node is None:
            return None
        
        return self.clone(node, [None] * 100)
        
    def clone(self, node: 'Node', visited: List[Optional['Node']] = []) -> 'Node':
        
        newNode = Node(node.val)
        cloned_neighbors = []
        
        if visited[node.val - 1] is not None:
            return visited[node.val - 1]
        
        tmp = node.neighbors
        node.neighbors = None
        
        
        visited[newNode.val - 1] = newNode
        
        if tmp is not None:
            
            for neighbor in tmp:
                cloned = self.clone(neighbor, visited)
                cloned_neighbors.append(cloned)

            node.neighbors = tmp
        
            newNode.neighbors = cloned_neighbors
            
        
        return newNode