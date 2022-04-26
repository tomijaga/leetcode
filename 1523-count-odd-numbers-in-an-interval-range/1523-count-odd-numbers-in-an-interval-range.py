def isEven(n: int) -> int:
    return n %2 == 0

class Solution:
    def countOdds(self, low: int, high: int) -> int:
        diff = high - low
        
        if isEven(low) and isEven(high) and True:
            return (int)(diff/2)
        else:
            return (int)(diff/2) + 1
