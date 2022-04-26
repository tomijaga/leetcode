class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        
        if len(nums) < 4:
            return []
        
        s = set()
        nums.sort()
        
        for i in range(0, len(nums) - 3):
            for j in range(i+1, len(nums) - 2):
                left = j + 1
                right = len(nums) -1
                
                while(left < right):
                    
                    sum = nums[i] + nums[j] + nums[left] + nums[right]
                    if (sum > target):
                        right-=1
                    elif (sum < target):
                        left += 1
                    else:
                        s.add((nums[i] , nums[j], nums[left] , nums[right]))
                        left += 1
        return [e for e in s]