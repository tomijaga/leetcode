class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        odd = 3
        for n in arr:
            if n % 2 == 1:
                odd-=1
            else:
                odd = 3
            
            if odd == 0:
                return True
        
        return False