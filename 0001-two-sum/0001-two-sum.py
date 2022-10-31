class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        
        obj = {}
        
        for i in range(0, len(nums)):
            numberWeAreLookingFor = target - nums[i]
            
            if numberWeAreLookingFor in obj:
                j = obj[numberWeAreLookingFor]
                return [i, j]
            
            obj[nums[i]] = i
            
        return []