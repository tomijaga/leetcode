/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class BSTIterator {
    Stack<TreeNode> stack;
    
    public BSTIterator(TreeNode root) {
        stack = new Stack<TreeNode>();
        putAll(root);
    }
    
    public void putAll(TreeNode root){
        TreeNode node = root;
        
        while(node != null){
            // System.out.println(node.val);
            stack.push(node);
            node = node.left;
        }
    }       
    
    public int next() {
        if (stack.size() > 0){
            TreeNode node = stack.pop();
            
            if (node.right != null){
                putAll(node.right);
            }
            
            return node.val;
        }
        
        return -1;
    }
    
    public boolean hasNext() {
        return stack.size() > 0;
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * BSTIterator obj = new BSTIterator(root);
 * int param_1 = obj.next();
 * boolean param_2 = obj.hasNext();
 */