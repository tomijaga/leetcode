
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        s = set([]);
        nums.sort()
        
        for i in range(0, len(nums) - 2):
            j = i + 1
            k = len(nums) - 1
            
            while(j<k):
                arr = [nums[i],  nums[j], nums[k]]
                arr_sum = sum(arr)

                if arr_sum > 0:
                    k-=1
                elif arr_sum < 0:
                    j+=1
                else:
                    s.add(tuple(arr))
                    k-=1
                    j+=1
        
        return [arr for arr in s]