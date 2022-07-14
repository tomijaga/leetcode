class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        preorder[::] = reversed(preorder)
        return build(preorder, inorder)
    
def build(preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
    if len(inorder) == 0 or len(preorder) == 0:
        return None
    
    node_val = preorder.pop()
    node_i = inorder.index(node_val)
    
    # print(node_i)
    node = TreeNode(node_val)
    
    left = inorder[:node_i]
        
    right = []
    if node_i < len(inorder) - 1:
        right = inorder[node_i+1:]
    
    
    node.left = build(preorder, left)
    
    node.right = build(preorder, right)
    
        
    return node