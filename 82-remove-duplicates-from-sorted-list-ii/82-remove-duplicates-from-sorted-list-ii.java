/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */

/// Iteration over recursion
class Solution {
    public ListNode deleteDuplicates(ListNode head) {
        if (head == null){
            return null;
        }
        
        ListNode dummy = new ListNode(-1); 
        dummy.next = head;
        ListNode curr = head;
        ListNode store = dummy;
        
        while (curr != null){
            while (curr.next != null && curr.next.val == curr.val){
                curr = curr.next;
            }
            
            if (store.next == curr){
                store = curr;
            }else{
                store.next = curr.next;
            }
            
            curr = curr.next;
            
        }
        
        return dummy.next;
    }
}