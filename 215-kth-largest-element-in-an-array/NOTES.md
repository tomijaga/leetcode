There are three different ways to solve this problem:
​
- Sort the list in descending order and return kth element
- Add all the elements to a min binary heap/ priority queue and pop the smallest element if the length of the heap is larger than k. Once all the elements have been added return the smallest element in the heap
- Use the selection algorithm to partition the array