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
        ListNode travA = headA;
        ListNode travB = headB;
        
        while (travA!= null || travB!= null){
            if (travA == null){
                travA = headB;
            }
            
            if (travB == null){
                travB = headA;
            }
            
            if (travA == travB ){
                return travA;
            }

            travA = travA.next;
            travB = travB.next;
            
        }
        
        return null;
    }
}