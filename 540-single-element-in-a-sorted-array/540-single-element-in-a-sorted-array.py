class Solution:
    def singleNonDuplicate(self, nums: List[int]) -> int:
        l = 0
        r = len(nums) - 1
        
        while (l < r):
            mid = floor(l + (r - l)/2)
            
            n = nums[mid]
            
            if mid % 2 == 0:
                if nums[mid + 1] == n:
                    l = mid + 1
                else:
                    r = mid
            else:
                if nums[mid + 1] == n:
                    r = mid
                else:
                    l = mid + 1
                    
        return nums[r]