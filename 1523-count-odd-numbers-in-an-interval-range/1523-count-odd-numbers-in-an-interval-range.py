def isOdd(n: int) -> int:
    return n %2 == 1

class Solution:
    def countOdds(self, low: int, high: int) -> int:
        diff = high - low
        
        if isOdd(low) == isOdd(high):
            if not isOdd(low):
                return (int)(diff/2)
            else:
                return (int)(diff/2) + 1
        else:
            return (int)(diff/2) + 1
