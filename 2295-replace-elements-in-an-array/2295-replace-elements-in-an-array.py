class Solution:
    def arrayChange(self, nums: List[int], operations: List[List[int]]) -> List[int]:
        map = {}
        
        for (i, n) in enumerate(nums):
            map[n] = i
            
        for [old, new] in (operations):
            i = map.pop(old)
            map[new] = i
            nums[i] = new
        
        return nums
        