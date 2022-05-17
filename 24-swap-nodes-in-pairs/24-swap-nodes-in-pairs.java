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
class Solution {
    public ListNode swapPairs(ListNode head) {
        if (head != null) {
            if (head.next != null){
                ListNode y = head.next;
                ListNode nextPair = y.next;
                
                y.next = head;
                head.next = swapPairs(nextPair);
                head = y;
            }
        }
        
        return head;
    }
}