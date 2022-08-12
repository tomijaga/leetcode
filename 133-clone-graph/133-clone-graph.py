class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        if node is None:
            return None
        
        return self.clone(node, {})
        
    def clone(self, node: 'Node', reference: Dict[int, 'Node']) -> 'Node':
        
        newNode = Node(node.val)
        cloned_neighbors = []
        
        if node.val - 1 in reference:
            return reference[node.val - 1]
        
        tmp = node.neighbors
        node.neighbors = None
        
        
        reference[newNode.val - 1] = newNode
        
        if tmp is not None:
            
            for neighbor in tmp:
                cloned = self.clone(neighbor, reference)
                cloned_neighbors.append(cloned)

            node.neighbors = tmp
        
            newNode.neighbors = cloned_neighbors
            
        
        return newNode