This is a problem that has many solutions but in this iteration we are trying to find the optimal solution.
The question requires that we code a solutions that solves this problem in `log(n)` time and with only `O(1)` extra space.
​
The idea for this question is to rethink how we look at the array.
Imagine the array with duplicated values for all its elements it would, for example:
​
`[1, 1, 2, 2, 3, 3, 4, 4, 5, 5]`
​
When we replace the values with only bits we can see a pattern
`[0, 1, 0, 1, 0, 1, 0, 1, 0, 1,]`
​
Let `0` be an even index in the array and `1` be an odd index in the array.
The duplicate values start at the bit `0` and end at `1`
​
Now imagine what happens if one of the duplicated is removed:
`[1, 1, 2, 2, 3, 3, 4, 5, 5]`
`[0, 1, 0, 1, 0, 1, 0, 1, 0,]`
​
The bit array becomes skewed.
The sequence shifts so that any duplicate sequence after the single value `4` starts from `1` and ends at `0`:
`[5, 5]`
`[1, 0]`
​
Now we can split the array into two parts the sequence `<=4` and the sequence `>4`
Using this knowledge we can come up with a solution using binary search.
​
​