class Solution:
    def search(self, nums: List[int], target: int) -> int:
        left = 0
        right = len(nums)
        
        while(left< right):
            mid = (int)(left + (right - left)/2)
            
            
            n = nums[mid]
            
            if (n > target):
                right = mid
            elif (n < target):
                left = mid + 1
            else: 
                return mid
        return -1