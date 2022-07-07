class Solution:
    def sortArrayByParity(self, nums: List[int]) -> List[int]:
        pivot = 0
        
        for (i, n) in enumerate(nums):
            if n % 2 == 0:
                swap(nums, pivot, i)
                pivot+=1
        
        return nums
            
def swap(nums: List[int], i:int, j: int):
    tmp = nums[i]
    nums[i] = nums[j]
    nums[j] = tmp