​
- time complexity : O(N)
​
- space complexity : O(N)
where N is the length of colors
​
This is similar to the [calculate product without self question]() where we have to loop throug the `colors` array twice. The first one from the left to the right and the second one in reverse.
​
We initialize an array, `prev`  with 3 elements representing the indices of the 3 different colors.
During the first iteration we replace the value at `prev[color]` with the current index and then calculate the difference of each color in `prev` with the current index and place the new array, `diff` in `distances`.
​
Next we iterate the `colors` array in reverse to store the distances from the current index in the array with the most recent color from the right. Then we compare this value with the value from the previous iteration and store the min.
​
Our array, `distances`, should now have the min distance of each color from the current index.
​
Now we loop through the `queries` array and lookup the index and color in our `distances` array. If the value is the same as the default value then we know the color does not exist in the array and we can push `-1` into the `res` array. If not, we push the values into `res`.