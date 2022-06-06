/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        if (headA == null || headB == null){
            return null;
        }
        
        ListNode travA = headA;
        ListNode travB = headB;
        
        while (travA!= null || travB!= null){
            if (travA == travB ){
                return travA;
            }

            travA = travA == null ? headB : travA.next;
            travB = travB == null ? headA : travB.next;
            
        }
        
        return null;
    }
}