def isOdd(elem: int):
    return elem %2 ==1

class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        
        numOds = 0
        
        for e in arr:
            if e % 2 == 1:
                numOds += 1
            else:
                numOds = 0
                
            if numOds == 3:
                return True
            
        return False