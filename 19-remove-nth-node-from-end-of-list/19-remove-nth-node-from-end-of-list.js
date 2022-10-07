var removeNthFromEnd = function(head, n) {
    let curr = head;
    let length = 0;
    
    while (curr){
        length +=1;
        curr = curr.next;
    }

    let target = (length - n);
    
    curr = head;
    
    if (target == 0){
        return curr.next
    }
    
    let i = 1;
    
    
    while (i < target){
        curr = curr.next
        i+=1;
    }
    if (curr && curr.next){
        curr.next = curr.next.next
    }
    
    return head
};