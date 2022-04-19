def isOdd(elem: int):
    return elem %2 ==1

class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        
        l = len(arr)
        while l > 2:
            if isOdd(arr[l -1]) and isOdd(arr[l-2]) and isOdd(arr[l-3]):
                return True
            l-=1
        return False