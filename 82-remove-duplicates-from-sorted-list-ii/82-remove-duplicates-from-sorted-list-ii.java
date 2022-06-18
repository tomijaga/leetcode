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
        ListNode trav = head;
        head = null;
        ListNode store = null;
        
        boolean dup = false;
        
        while (true){
            while (trav.next != null && trav.next.val == trav.val){
                dup = true;
                trav = trav.next;
            }
            
            if (dup){
                trav = trav.next;
                dup = false;
            }else{
                if (head == null){
                    head = trav;
                    store = head;
                }else{
                    store.next = trav;
                    store = store.next;
                }
                
                trav = trav.next;
            }
            
            if (trav == null){
                if (store != null){
                    store.next = null;
                }
                break;
            }
        }
        
        return head;
    }
}