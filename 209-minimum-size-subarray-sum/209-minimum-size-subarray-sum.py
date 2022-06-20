class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        i, sum, size, m = 0, 0, len(nums), len(nums) + 1
        
        for j in range(0, size):
            sum +=nums[j]
            while sum>=target:
                m = min(m, j - i + 1)
                sum -= nums[i]
                i+=1
                
        return 0 if (m == size + 1) else m
