class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        maxSum = nums[0]
        sum = 0
        
        for n in nums:
            if sum < 0:
                sum = 0
            sum +=n
            maxSum = max(sum, maxSum)

        return maxSum