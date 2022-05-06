class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res = []
        for i in range(0, len(nums) - 2):
            if i>0 and nums[i] == nums[i-1]:
                continue
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
                    # print([[i, j, k], arr])
                    res.append(arr)
                    while j< k and nums[j] == nums[j + 1]:
                        j+=1
                    else:
                        j+=1
                    
                    while j< k and nums[k] == nums[k-1]:
                        k-=1
                    else:
                        k-=1

        
        return res