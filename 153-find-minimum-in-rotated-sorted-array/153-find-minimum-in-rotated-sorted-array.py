class Solution:
    def findMin(self, nums: List[int]) -> int:
        min = nums[0]
        l, r = 0, len(nums)
        
        if nums[-1] >= min:
            return min
        
        while l < r:
            mid = l + floor((r - l)/2)
            n = nums[mid]
            
            if n < min:
                min = n
                r = mid
            elif n > min:
                l  = mid + 1
            else:
                return mid
            
        return min