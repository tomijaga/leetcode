from itertools import accumulate
class Solution:
    def pivotIndex(self, nums: List[int]) -> int:
        nums[::] = accumulate(nums)
        total = nums[-1]
        
        prev = 0
        for (i, n) in enumerate(nums):
            if i == 0:
                if total- n == 0:
                    return i
            elif i == len(nums) - 1:
                if prev == 0:
                    return i
            else:
                if prev == total - n:
                    return i
                
            prev = n
            
        return -1
                    
            