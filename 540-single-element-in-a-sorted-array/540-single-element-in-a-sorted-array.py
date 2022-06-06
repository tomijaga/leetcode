class Solution:
    def singleNonDuplicate(self, nums: List[int]) -> int:
        l = 0
        r = len(nums) - 1
        
        while (l < r):
            mid = floor(l + (r - l)/2)
            
            # reduce mid to even value
            mid = mid - 1 if mid % 2 == 1 else mid
            
            n = nums[mid]
            
            if nums[mid + 1] == n:
                # add 2 to avoid looping forever with an odd value
                l = mid + 2
            else:
                r = mid
                
        return nums[r]