class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        obj = {}
        for (i, n) in enumerate(nums):
            other = target - n
            
            if other in obj:
                return [i, obj[other]]
            else:
                obj[n] = i
            
        return [-1, -1]