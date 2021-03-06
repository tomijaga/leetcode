### Solution
My solution involves recursion and iterating over duplicate elements
​
​
We pass the head node through a while loop that consumes the duplicate elements until it gets to an element that is different.
​
​
Since this question requires that we remove all the node that occurs more than once we have to move past the current node if it entered the while loop. Entering the while loop indicates that the `curr` node and the `next` nodes were equal.
​
If there was indeed a duplicate we call the recursive function on the `next` node and return its result instead of the `curr` node.
If there wasn't any duplicates we call the recursive function on the `next` node and replace it with the result. Then we return the `curr` node
​
### Efficient solution
The most efficient solution would be to use two while loops for removing duplicates
​
- Create a dummy node
- Copy the `head` to the dummy nodes `.next` pointer
- Create a node to traverse the list (`curr` / `trav`)
- Create a while loop that checks if the `curr` node is null
- Nest another while loop that checks if there are duplicate values and consumes them
- Check if the node at the cloned node is the same
- Nest another while loop that checks if there are duplicate values and consumes them