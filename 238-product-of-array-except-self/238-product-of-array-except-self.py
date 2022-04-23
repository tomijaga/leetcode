class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        results = [1 for i in range(len(nums))]
        
        prod =1
        for i in range(0, len(nums) -1):
            n = nums[i]
            prod *= n
            
            results[i+1] = prod
        
        prod=1
        for i in reversed(range(1, len(nums))):
            n = nums[i]
            prod *= n
            
            results[i-1] *= prod
        
        
        return results