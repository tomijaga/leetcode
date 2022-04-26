import sys 

class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums.sort()
        
        closest = 3001
        
        for i in range(0, len(nums) -2):
            j = i+1
            k = len(nums) -1
            
            while(j < k):
                sum = nums[i] + nums[j] + nums[k]
                
                if abs(target - closest) > abs(target - sum):
                    closest = sum
                if (sum > target):
                    k-=1
                elif (sum < target):
                    j+=1
                else:
                    return sum
                
        return closest